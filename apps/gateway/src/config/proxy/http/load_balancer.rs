use pingora::lb::{
    discovery,
    selection::{Random, RoundRobin},
    Backend, Backends, LoadBalancer,
};
use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use tracing::debug;

use crate::config::source_config::{LoadBalancerAlgorithm, UpstreamConfig};

pub enum LoadBalancerEnum {
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
pub struct UpStreamLoadBalaner {
    pub name: String,
    pub load_balancer: LoadBalancerEnum,
}

impl UpStreamLoadBalaner {
    pub async fn from_upstream_config(upstreams: Vec<UpstreamConfig>) -> Vec<Self> {
        let mut upstream_load_balancers: Vec<UpStreamLoadBalaner> = vec![];
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
            let backends = Backends::new(discovery);
            // Do not remove update
            backends.update(|_| {}).await.unwrap();
            // Do not remove update

            if upstream.traffic_distribution_policy == LoadBalancerAlgorithm::RoundRobin {
                let lb = LoadBalancer::from_backends(backends);
                upstream_load_balancers.push(UpStreamLoadBalaner {
                    name: upstream.name,
                    load_balancer: LoadBalancerEnum::RoundRobin(lb),
                });
            } else if upstream.traffic_distribution_policy == LoadBalancerAlgorithm::Random {
                let lb = LoadBalancer::from_backends(backends);
                upstream_load_balancers.push(UpStreamLoadBalaner {
                    name: upstream.name,
                    load_balancer: LoadBalancerEnum::Random(lb),
                });
            }
        }

        upstream_load_balancers
    }

    pub fn get_backend(&self) -> Backend {
        let back_end: Backend = match &self.load_balancer {
            LoadBalancerEnum::RoundRobin(load_balancer) => load_balancer.select(b"", 256).unwrap(),
            LoadBalancerEnum::Random(load_balancer) => load_balancer.select(b"", 256).unwrap(),
        };
        back_end
    }
}
