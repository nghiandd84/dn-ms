use serde::{Deserialize, Serialize};

use super::inet_address::InetAddress;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancerAlgorithm {
    RoundRobin,
    Random,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpstreamNodeConfig {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpstreamConfig {
    pub name: String,
    pub default: bool,
    pub upstream_nodes: Vec<UpstreamNodeConfig>,
    pub traffic_distribution_policy: LoadBalancerAlgorithm,
}
