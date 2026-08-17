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
        interceptor::{execute_interceptors, Phase},
        state::GatewayStateStore,
    },
};

use super::ctx::HttpGatewayCtx;

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
}

impl Proxy {
    pub fn new(gateway_state_store: Arc<GatewayStateStore>) -> Proxy {
        Proxy {
            gateway_state_store,
        }
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
        let filter = match find_filter_config(gateway_config, session.ds_req_path()) {
            Ok(f) => f,
            Err(_) => {
                error!("Not found filter for path {}", session.ds_req_path());
                return Err(Error::new_str("Not found filter for path"));
            }
        };
        session.flush_path_and_query(&filter);
        let filter_name = filter.name.clone();
        session.set_filter(filter);

        let filter_interceptors = state.get_interceptors(Phase::Init, filter_name);
        let invalid_execute = execute_interceptors(&filter_interceptors, &mut session, &Phase::Init).await;
        match invalid_execute {
            Ok(success) => {
                debug!(
                    "Successfully executed early_request_filter interceptors with result {}",
                    success
                );
                if success {
                    let err = Error::new_str("Terminated by early_request_filter interceptor");
                    return Err(err.into());
                }
                return Ok(());
            }
            Err(e) => {
                error!("Error executing early_request_filter interceptors: {:?}", e);
                return Err(Error::new_str("Error in early_request_filter interceptor"));
            }
        }
    }

    async fn request_filter(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>> {
        let filter = ctx.filter.clone().unwrap();
        debug!("request_filter - Filter Name: {}", filter.name);
        let mut session = session::Session::build(Phase::RequestFilter, _session, ctx);

        let state = self.gateway_state_store.get_state();
        let filter_interceptors = state.get_interceptors(Phase::RequestFilter, filter.name.clone());
        debug!(
            "Executing request_filter interceptors with length {}",
            filter_interceptors.len()
        );
        let invalid_execute = execute_interceptors(&filter_interceptors, &mut session, &Phase::RequestFilter).await;
        match invalid_execute {
            Ok(success) => {
                debug!(
                    "Successfully executed request_filter interceptors with result {}",
                    success
                );
                if success {
                    let err = Error::new_str("Terminated by request_filter interceptor");
                    return Err(err.into());
                }
                return Ok(false);
            }
            Err(e) => {
                error!("Error executing request_filter interceptors: {:?}", e);
                return Err(Error::new_str("Error in request_filter interceptor"));
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

        let upstream_load_balancers = state.upstream_load_balancers();
        let upstream_load_balancer = upstream_load_balancers
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

        let state = self.gateway_state_store.get_state();
        let filter_interceptors =
            state.get_interceptors(Phase::PostUpstreamResponse, filter.name.clone());
        let invalid_execute = execute_interceptors(&filter_interceptors, &mut session, &Phase::PostUpstreamResponse).await;
        match invalid_execute {
            Ok(success) => {
                debug!(
                    "Successfully executed response_filter interceptors with result {}",
                    success
                );
                if success {
                    let err = Error::new_str("Terminated by response_filter interceptor");
                    return Err(err.into());
                }
            }
            Err(e) => {
                error!("Error executing response_filter interceptors: {:?}", e);
                let error = Error::new_str("Error in response_filter interceptor");
                return Err(error);
            }
        }
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
