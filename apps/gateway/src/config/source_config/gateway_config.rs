use serde::{Deserialize, Serialize};

use super::{
    downstream_config::DownstreamConfig, inet_address::InetAddress, router_config::RouterConfig,
    upstream_config::UpstreamConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    // TODO: use auto generated name
    pub name: String,
    // TODO: add type = HTTP, TCP, SMTP, etc
    pub bind_addresses: Vec<InetAddress>,
    // pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,
    pub routers: Vec<RouterConfig>,
    // #[serde(default)]
    // pub interceptors: Vec<InterceptorConfig>,

    // #[serde(default)]
    // pub filters: Vec<Query>,
}
