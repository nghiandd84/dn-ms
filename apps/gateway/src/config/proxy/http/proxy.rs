use async_trait::async_trait;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::{baggage::BaggageExt, global};
use opentelemetry_sdk::propagation::BaggagePropagator;
use pingora::{prelude::HttpPeer, upstreams::peer::Peer, Error};
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tracing::{debug, error, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    config::{
        proxy::http::{
            session,
            tracing::{PingoraHeaderExtractor, PingoraHeaderInjector},
        },
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
                let interceptor_name = format!("{:?}", interceptor.interceptor_type());
                let is_match_phase = interceptor.phase_mask() & phase.mask() != 0;
                let default_filter = String::from("");
                let interceptor_filter = interceptor.filter().as_ref().unwrap_or(&default_filter);
                let is_match_filter = *interceptor_filter == filter_name;
                debug!(
                    "get_interceptors filter_name: {} interceptor_name {} is_match_phase: {} is_match_filter: {}",
                    filter_name, interceptor_name, is_match_phase, is_match_filter
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
        ctx.set_span_context(span.context());

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

    async fn request_filter(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>> {
        let filter = ctx.filter.clone().unwrap();
        debug!("request_filter - Filter Name: {}", filter.name);
        let mut session = session::Session::build(Phase::RequestFilter, _session, ctx);
        let filter_interceptors = self.get_interceptors(Phase::RequestFilter, filter.name.clone());
        debug!(
            "Executing request_filter interceptors with length {}",
            filter_interceptors.len()
        );
        let invalid_execute = execute_interceptors(&filter_interceptors, &mut session).await;
        match invalid_execute {
            Ok(success) => {
                debug!(
                    "Successfully executed request_filter interceptors with result {}",
                    success
                );
                return Ok(success);
            }
            Err(e) => {
                error!("Error executing request_filter interceptors: {:?}", e);
                return Ok(true);
            }
        }
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
        debug!("response_filter - Filter Name: {}", filter.name);
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
        let mut session = session::Session::build(Phase::PreUpstreamRequest, _session, ctx);
        let context = session.get_span_context();
        let context = context.clone().unwrap();
        debug!("Current Context: {:?}", context);
        let baggage = context.baggage();
        debug!("Baggage data {:?}", baggage);

        global::get_text_map_propagator(|prop| {
            prop.inject_context(&context, &mut PingoraHeaderInjector(upstream_request))
        });
        let propagator = BaggagePropagator::new();
        let mut fields = HashMap::new();

        propagator.inject_context(&context, &mut fields);
        if let Some(baggage_value) = fields.get("baggage") {
            upstream_request
                .insert_header("baggage", baggage_value)
                .map_err(|e| {
                    pingora_core::Error::because(
                        pingora_core::ErrorType::HTTPStatus(500),
                        "Failed to inject baggage header",
                        e,
                    )
                })?;
        }

        let _up = session.upstream_request(upstream_request);
        let _plush = session.flush_us_req_header();

        Ok(())
    }
}
