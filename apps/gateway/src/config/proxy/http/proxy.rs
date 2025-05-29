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
use tracing_subscriber::field::debug;

use crate::{
    config::{proxy::http::session, source_config::LoadBalancerAlgorithm},
    gateway::state::GatewayStateStore,
};

use super::ctx::HttpGatewayCtx;

enum LoadBalancerEnum {
    RoundRobin(LoadBalancer<RoundRobin>),
    Random(LoadBalancer<Random>),
}

impl Debug for LoadBalancerEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundRobin(arg0) => f.debug_tuple("RoundRobin").finish(),
            Self::Random(arg0) => f.debug_tuple("Random").finish(),
        }
    }
}

#[derive(Debug)]
struct UpStreamLoadBalaner {
    name: String,
    load_balancer: LoadBalancerEnum,
}

#[derive(Clone)]
pub struct Proxy {
    gateway_state_store: Arc<GatewayStateStore>,
    upstream_load_balancers: Arc<Vec<UpStreamLoadBalaner>>,
}

impl Proxy {
    pub fn build(gateway_state_store: Arc<GatewayStateStore>) -> Proxy {
        let mut upstream_load_balancers: Vec<UpStreamLoadBalaner> = vec![];
        let state = gateway_state_store.get_state();
        let gateway_config = state.gateway_config();
        let upstreams = gateway_config.clone().upstreams;
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
        HttpGatewayCtx::default()
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let request_id = uuid::Uuid::new_v4().to_string();
        ctx.request_id = Some(request_id.clone());
        let _ = session
            .req_header_mut()
            .insert_header("X-Request-Id", &request_id);
        Ok(())
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        debug!("Upstream {:?}", self.upstream_load_balancers);
        let path = session.req_header().uri.path();
        let current_path_and_query = session
            .req_header()
            .uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");
        let mut new_path_str = current_path_and_query.to_string();
        debug!("Path {}", path);

        // TODO Use config data to load gateway
        // Use false -> http  or true -> https
        let tls = false;

        if path.starts_with("/api/bakery") {
            debug!("Start with bakery");
            new_path_str.replace_range(0.."/api/bakery".len(), "");
            debug!("New Path {}", new_path_str);
            let uri = new_path_str.parse::<Uri>().unwrap();
            session.req_header_mut().set_uri(uri);
            let peer = HttpPeer::new("127.0.0.1:5002", tls, "bakery".to_string());
            return Ok(Box::new(peer));
        }
        let peer = HttpPeer::new("127.0.0.1:5001", tls, "auth".to_string());
        Ok(Box::new(peer))
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
