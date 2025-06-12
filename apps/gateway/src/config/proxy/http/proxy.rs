use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    sync::Arc,
};

use async_trait::async_trait;

use http::Uri;
use pingora::lb::{
    discovery,
    selection::{Random, RoundRobin},
    Backend, Backends, LoadBalancer,
};
use pingora::{prelude::HttpPeer, Error};
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use tracing::debug;

use crate::{
    config::{
        proxy::http::session,
        source_config::{find_filter_config, find_router_config, LoadBalancerAlgorithm},
    },
    gateway::state::GatewayStateStore,
};

use super::ctx::HttpGatewayCtx;
use super::load_balancer::{LoadBalancerEnum, UpStreamLoadBalaner};

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
    upstream_load_balancers: Arc<Vec<UpStreamLoadBalaner>>,
}

impl Proxy {
    pub fn build(gateway_state_store: Arc<GatewayStateStore>) -> Proxy {
        let state = gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let upstreams = gateway_config.clone().upstreams;
        let upstream_load_balancers: Vec<UpStreamLoadBalaner> =
            UpStreamLoadBalaner::from_upstream_config(upstreams);
        /*
        for upstream in upstreams {
            let mut backends: BTreeSet<Backend> = BTreeSet::new();
            for upstream_node in upstream.upstream_nodes {
                let mut back_end = Backend::new_with_weight(
                    upstream_node.address.get_formatted_address().as_str(),
                    upstream_node.weight.unwrap_or(1) as usize,
                )
                .unwrap();
                let mut ext_data: HashMap<String, bool> = std::collections::HashMap::new();
                ext_data.insert("tls".to_string(), upstream_node.tls);
                back_end.ext.insert(ext_data);
                backends.insert(back_end);
            }
            let discovery = discovery::Static::new(backends);

            if upstream.traffic_distribution_policy == LoadBalancerAlgorithm::RoundRobin {
                let lb: LoadBalancer<RoundRobin> =
                    LoadBalancer::from_backends(Backends::new(discovery));
                upstream_load_balancers.push(UpStreamLoadBalaner {
                    name: upstream.name,
                    load_balancer: LoadBalancerEnum::RoundRobin(lb),
                });
            } else if upstream.traffic_distribution_policy == LoadBalancerAlgorithm::Random {
                let lb: LoadBalancer<Random> =
                    LoadBalancer::from_backends(Backends::new(discovery));
                upstream_load_balancers.push(UpStreamLoadBalaner {
                    name: upstream.name,
                    load_balancer: LoadBalancerEnum::Random(lb),
                });
            }
        }
        */
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
        debug!("Upstream {:?}", self.upstream_load_balancers);

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

        // TODO Use config data to load gateway
        // Use false -> http  or true -> https
        let tls = false;

        // if path.starts_with("/api/bakery") {
        // debug!("Start with bakery");
        // new_path_str.replace_range(0.."/api/bakery".len(), "");
        // debug!("New Path {}", new_path_str);
        // let uri = new_path_str.parse::<Uri>().unwrap();
        // _session.req_header_mut().set_uri(uri);
        let peer = HttpPeer::new("127.0.0.1:5202", tls, "bakery".to_string());
        Ok(Box::new(peer))
        
        // }
        // let peer = HttpPeer::new("127.0.0.1:5101", tls, "auth".to_string());
        // Ok(Box::new(peer))
        /*
        let backend = self.load_balancer.select(b"", 256).unwrap();
        debug!("backend {:?}", backend);
        let peer = Box::new(HttpPeer::new(backend, false, "bakery".to_string()));
        Ok(peer)
         */
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
