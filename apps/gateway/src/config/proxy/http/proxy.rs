use async_trait::async_trait;
use http::{HeaderName, HeaderValue};
use pingora::{prelude::HttpPeer, upstreams::peer::Peer, Error};
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};
use tracing::{debug, info, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    config::{
        proxy::http::session,
        source_config::{find_filter_config, find_router_config},
    },
    gateway::{
        interceptor::{execute_interceptors, Interceptor, Phase},
        interceptor_builder::{utils::build_interceptors, InterceptorBuilderRegistry},
        state::GatewayStateStore,
    },
};

use super::ctx::HttpGatewayCtx;
use super::load_balancer::UpStreamLoadBalaner;

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
    upstream_load_balancers: Arc<Vec<UpStreamLoadBalaner>>,
    insterceptors: Vec<Arc<dyn Interceptor>>,
}

impl Proxy {
    pub async fn build(gateway_state_store: Arc<GatewayStateStore>) -> Proxy {
        let state = gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let upstreams = gateway_config.clone().upstreams;
        let upstream_load_balancers: Vec<UpStreamLoadBalaner> =
            UpStreamLoadBalaner::from_upstream_config(upstreams).await;
        let interceptor_builder_registry = InterceptorBuilderRegistry::build();
        let interceptors =
            build_interceptors(gateway_config, &interceptor_builder_registry).unwrap();
        debug!("Interceptor len {}", interceptors.len());

        Proxy {
            gateway_state_store,
            upstream_load_balancers: Arc::new(upstream_load_balancers),
            insterceptors: interceptors,
        }
    }

    pub fn get_interceptors(&self, phase: Phase, filter_name: String) -> Vec<Arc<dyn Interceptor>> {
        let r = self
            .insterceptors
            .iter()
            .filter(|interceptor| {
                let is_match_phase = interceptor.phase_mask() & phase.mask() != 0;
                let default_filter = String::from("");
                let interceptor_filter = interceptor.filter().as_ref().unwrap_or(&default_filter);
                let is_match_filter = *interceptor_filter == filter_name;
                debug!(
                    "get_interceptors filter_name: {} is_match_phase: {} is_match_filter: {}",
                    filter_name, is_match_phase, is_match_filter
                );
                is_match_phase && is_match_filter
            })
            .cloned()
            .collect();
        r
    }
}

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = HttpGatewayCtx;

    fn new_ctx(&self) -> Self::CTX {
        HttpGatewayCtx::new()
    }

    async fn request_filter(
        &self,
        psession: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<bool, Box<Error>> {
        Ok(false)
    }

    async fn early_request_filter(
        &self,
        psession: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<(), Box<Error>> {
        debug!("early_request_filter -----------------");
        let parent_cx = global::get_text_map_propagator(|prop| {
            let result = prop.extract(&PingoraHeaderExtractor(psession.req_header()));
            result
        });

        let request_name = format!(
            "request {} {}",
            psession.req_header().method,
            psession.req_header().uri,
        );
        let span = info_span!("request", otel.name = %request_name);
        let _ = span.set_parent(parent_cx);
        let _gaurd = span.enter();
        ctx.span_context = Some(span.context());

        let mut session = session::Session::build(Phase::Init, psession, ctx);

        let state = self.gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let filter = find_filter_config(gateway_config, session.ds_req_path())
            .expect(format!("Not found filter for path {}", session.ds_req_path()).as_str());
        let filter_interceptors = self.get_interceptors(Phase::Init, filter.name.clone());

        let _execute = execute_interceptors(&filter_interceptors, &mut session).await;

        session.flush_path_and_query(&filter);
        ctx.set_filter(filter);

        Ok(())
    }

    async fn upstream_peer(
        &self,
        psession: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let _session = session::Session::build(Phase::UpstreamPeerSelection, psession, ctx);
        let state = self.gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let filter = ctx.filter.as_ref().unwrap();
        let router_config = find_router_config(gateway_config, filter).unwrap();
        let upstream_name = router_config.upstream;
        let upstream_load_balancer = self
            .upstream_load_balancers
            .as_ref()
            .iter()
            .find(|us_balance| us_balance.name == upstream_name)
            .unwrap();
        let back_end = upstream_load_balancer.get_backend();
        debug!("back_end {:?}", back_end);
        let ext = back_end.ext.get::<HashMap<String, bool>>().unwrap();

        let tls = ext.get("tls").unwrap();
        let mut peer = HttpPeer::new(&back_end.addr, *tls, upstream_name);

        if filter.timeout.is_some() {
            let timeout = filter.timeout.unwrap();
            debug!("Set timeout for peer: {} seconds", timeout);
            let option = peer.get_mut_peer_options().unwrap();
            option.read_timeout = Some(Duration::from_secs(timeout));
            option.write_timeout = Some(Duration::from_secs(timeout));
        }

        Ok(Box::new(peer))
    }

    async fn response_filter(
        &self,
        psession: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let filter = ctx.filter.clone().unwrap();
        debug!("RequestFilter - Filter Name: {}", filter.name);
        let mut session = session::Session::build(Phase::PostUpstreamResponse, psession, ctx);
        session.upstream_response(upstream_response);
        let filter_interceptors =
            self.get_interceptors(Phase::PostUpstreamResponse, filter.name.clone());
        let _execute = execute_interceptors(&filter_interceptors, &mut session).await;
        let _flush = session.flush_ds_res_header().await;
        Ok(())
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<(), pingora_core::BError>
    where
        Self::CTX: Send + Sync,
    {
        let context = ctx.span_context.as_ref().unwrap().clone();
        debug!("Current Context: {:?}", context);
        let mut session = session::Session::build(Phase::PreUpstreamRequest, _session, ctx);

        global::get_text_map_propagator(|prop| {
            prop.inject_context(&context, &mut PingoraHeaderInjector(upstream_request))
        });

        let _up = session.upstream_request(upstream_request);
        let _plush = session.flush_us_req_header();

        Ok(())
    }
}

use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
};
// Helper for Extraction (Reading incoming headers)
struct PingoraHeaderExtractor<'a>(&'a pingora_http::RequestHeader);
impl<'a> Extractor for PingoraHeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        let data = self.0.headers.get(key).and_then(|v| v.to_str().ok());
        debug!(
            "Extracting header: {} with  value {}",
            key,
            data.unwrap_or("Unknown")
        );
        data
    }
    fn keys(&self) -> Vec<&str> {
        debug!("Getting all header keys");
        self.0.headers.iter().map(|(k, _)| k.as_str()).collect()
    }
}

// Helper for Injection (Writing outgoing headers)
struct PingoraHeaderInjector<'a>(&'a mut pingora_http::RequestHeader);
impl<'a> Injector for PingoraHeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        debug!("Injecting header: {}: {}", key, value);
        let header_value = HeaderValue::from_str(&value).unwrap();
        let header_name = HeaderName::from_str(key).unwrap();
        let _ = self.0.insert_header(header_name, header_value);
    }
}
