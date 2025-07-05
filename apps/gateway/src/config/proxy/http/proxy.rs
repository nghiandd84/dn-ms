use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use pingora::{prelude::HttpPeer, Error};
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use tracing::debug;

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
        debug!("Proxy new_ctx");
        HttpGatewayCtx::new()
    }

    async fn early_request_filter(
        &self,
        psession: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<(), Box<Error>> {
        debug!("early_request_filter -----------------");
        

        let mut session = session::Session::build(Phase::Init, psession, ctx);

        let state = self.gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let filter = find_filter_config(gateway_config, session.ds_req_path()).unwrap();
        let filter_interceptors = self.get_interceptors(Phase::Init, filter.name.clone());

        execute_interceptors(&filter_interceptors, &mut session).await;

        session.flush_path_and_query(&filter);
        ctx.set_filter(filter);

        Ok(())
    }

    async fn upstream_peer(
        &self,
        psession: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        debug!("upstream_peer ------------------");
        debug!("Current Ctx {:?}", ctx);

        let mut session = session::Session::build(Phase::UpstreamPeerSelection, psession, ctx);
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
        let peer = HttpPeer::new(&back_end.addr, *tls, upstream_name);
        Ok(Box::new(peer))
    }

    async fn response_filter(
        &self,
        psession: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        debug!("response_filter");
        let mut session = session::Session::build(Phase::PostUpstreamResponse, psession, ctx);
        session.upstream_response(upstream_response);
        session.flush_ds_res_header().await;
        Ok(())
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        _upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), pingora_core::BError>
    where
        Self::CTX: Send + Sync,
    {
        let mut session = session::Session::build(Phase::PreUpstreamRequest, _session, _ctx);
        session.upstream_request(_upstream_request);
        session.flush_us_req_header();

        Ok(())
    }
}
