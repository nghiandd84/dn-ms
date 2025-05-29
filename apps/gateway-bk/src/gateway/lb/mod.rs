use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

use async_trait::async_trait;
use pingora::lb::{
    discovery, selection::{algorithms::RoundRobin, weighted::Weighted}, Backend, Backends, LoadBalancer
};

use tokio::sync::RwLock;

use crate::{
    config::source_config::UpstreamConfig,
    error::{DakiaError, DakiaResult},
    shared::registry::Registry,
};

type LB = LoadBalancer<Weighted<RoundRobin>>;

pub struct LoadBalancerRegistry {
    registry: RwLock<HashMap<String, Arc<LB>>>,
}

#[async_trait]
impl Registry<Arc<LB>> for LoadBalancerRegistry {
    async fn register(&self, key: String, lb: Arc<LB>) {
        let mut write_guard = self.registry.write().await;
        write_guard.insert(key, lb);
    }

    async fn get(&self, key: &str) -> DakiaResult<Option<Arc<LB>>> {
        let read_guard = self.registry.read().await;
        let arc_lb = read_guard.get(key).ok_or(DakiaError::i_explain(format!(
            "Load balancer {key:?} not found."
        )))?;
        Ok(Some(arc_lb.clone()))
    }
}

impl LoadBalancerRegistry {
    pub fn build() -> Self {
        Self {
            registry: RwLock::new(HashMap::new()),
        }
    }
}

pub fn build_lb(upstream_config: &UpstreamConfig) -> DakiaResult<LB> {
    let addrs: Vec<String> = upstream_config
        .upstream_nodes
        .iter()
        .map(|node| node.address.get_formatted_address())
        .collect();
    
    let b1 = Backend::new("1.1.1.1:80").unwrap();
    let mut b2 = Backend::new("1.0.0.1:80").unwrap();
    b2.weight = 10; // 10x than the rest
    let b3 = Backend::new("1.0.0.255:80").unwrap();
    let backends = BTreeSet::from_iter([b1.clone(), b2.clone(), b3.clone()]);
    let discovery = discovery::Static::new(backends);
    let backends = Backends::new(discovery);
    let lb: LoadBalancer<Weighted<RoundRobin>> = LoadBalancer::from_backends(backends);
    

    let lb: LoadBalancer<Weighted<RoundRobin>> = LoadBalancer::try_from_iter(addrs)?;
    Ok(lb)
}

pub type LbRegistryType = Arc<dyn Registry<Arc<LB>> + Send + Sync>;
