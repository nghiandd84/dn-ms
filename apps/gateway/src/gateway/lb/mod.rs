use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use pingora::lb::{
    selection::{algorithms::RoundRobin, weighted::Weighted},
    LoadBalancer,
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

    let lb: LoadBalancer<Weighted<RoundRobin>> = LoadBalancer::try_from_iter(addrs)?;
    Ok(lb)
}

pub type LbRegistryType = Arc<dyn Registry<Arc<LB>> + Send + Sync>;
