use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use pingora::{prelude::HttpPeer, Error};
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use tracing::debug;

use crate::{
    config::{
        proxy::http::session,
        source_config::{find_filter_config, find_router_config},
    },
    gateway::state::GatewayStateStore,
};

use super::ctx::HttpGatewayCtx;
use super::load_balancer::UpStreamLoadBalaner;

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
    upstream_load_balancers: Arc<Vec<UpStreamLoadBalaner>>,
}

impl Proxy {
    pub async fn build(gateway_state_store: Arc<GatewayStateStore>) -> Proxy {
        let state = gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let upstreams = gateway_config.clone().upstreams;
        let upstream_load_balancers: Vec<UpStreamLoadBalaner> =
            UpStreamLoadBalaner::from_upstream_config(upstreams).await;
        Proxy {
            gateway_state_store,
            upstream_load_balancers: Arc::new(upstream_load_balancers),
        }
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
        let request_id = ctx.request_id.as_ref().unwrap();
        let _ = psession
            .req_header_mut()
            .insert_header("X-Request-Id", request_id);
        let mut session = session::Session::build(psession, ctx);
        let state = self.gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let filter = find_filter_config(gateway_config, session.ds_req_path()).unwrap();
        session.flush_path_and_query(&filter);

        ctx.set_filter(filter);

        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut HttpGatewayCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        debug!("upstream_peer ------------------");
        debug!("Current Ctx {:?}", ctx);
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

        // debug!("upstream_load_balancer {:?}", upstream_load_balancer);
        let back_end = upstream_load_balancer.get_backend();
        debug!("back_end {:?}", back_end);
        let ext = back_end.ext.get::<HashMap<String, bool>>().unwrap();

        let tls = ext.get("tls").unwrap();
        let peer = HttpPeer::new(&back_end.addr, *tls, upstream_name);
        Ok(Box::new(peer))
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        if let Some(request_id) = &_ctx.request_id {
            let _ = response.insert_header("X-Request-Id", request_id);
        }
        Ok(())
    }
}
