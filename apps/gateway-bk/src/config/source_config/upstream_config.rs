use serde;

use crate::error::{DakiaError, DakiaResult};

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeSelectionAlgorithm {
    RoundRobin,
    Weighted,
    LeastConnection,
    IpHash,
    UrlHash,
    Random,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrafficDistributionPolicy {
    node_selection_algorithm: NodeSelectionAlgorithm,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamNodeConfig {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamConfig {
    pub name: String,
    pub default: bool,
    pub upstream_nodes: Vec<UpstreamNodeConfig>,
    pub traffic_distribution_policy: Option<TrafficDistributionPolicy>,
}
impl UpstreamConfig {
    pub fn find_upstream_node_config(&self, address: InetAddress) -> Option<&UpstreamNodeConfig> {
        self.upstream_nodes.iter().find(|node_config| {
            node_config.address.get_formatted_address() == address.get_formatted_address()
        })
    }

    pub fn find_upstream_node_config_or_err(
        &self,
        address: InetAddress,
    ) -> DakiaResult<&UpstreamNodeConfig> {
        let node_config = self.find_upstream_node_config(address);
        node_config.ok_or(DakiaError::create_unknown_context(
            crate::error::ImmutStr::Static("upstream node config not found".into()),
        ))
    }
}
