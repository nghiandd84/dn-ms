use std::error::Error;

use serde::{Deserialize, Serialize};

use super::{
    downstream_config::DownstreamConfig, filter::PathFilter, inet_address::InetAddress,
    router_config::RouterConfig, upstream_config::UpstreamConfig, Filter,
};

use crate::{config::source_config::InterceptorConfig, error::{Error as DnError, GatewayResult}};

// super::filter::PathFilter;

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    // TODO: use auto generated name
    pub name: String,
    // TODO: add type = HTTP, TCP, SMTP, etc
    pub bind_addresses: Vec<InetAddress>,
    // pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,
    pub routers: Vec<RouterConfig>,
    // #[serde(default)]
    pub interceptors: Vec<InterceptorConfig>,

    // #[serde(default)]
    pub filters: Vec<Filter>,
}

pub fn find_filter_config<'a>(gateway_config: &'a GatewayConfig, path: &'a str) -> GatewayResult<Filter> {
    let filter = gateway_config.filters.iter().find(|filter| {
        let path_filter = filter.path.as_ref().unwrap();

        match path_filter {
            PathFilter::StartWith { value } => path.starts_with(value),
            PathFilter::EndWith { value } => path.ends_with(value),
        }
    });
    if filter.is_none() {
        return Err(Box::new(DnError::from_str("Not found filter".to_owned())));
    }
    let filter = filter.unwrap();
    Ok(filter.clone())
}

pub fn find_router_config<'a>(
    gateway_config: &'a GatewayConfig,
    filter: &'a Filter,
) -> GatewayResult<RouterConfig> {
    let search_filter = filter.name.clone();
    for router_conifg in &gateway_config.routers {
        match &router_conifg.filter {
            Some(filter_name) => {
                if *filter_name == search_filter {
                    return Ok(router_conifg.clone());
                }
            }
            _ => {}
        }
    }

    Err(Box::new(DnError::from_str("Not found router".to_owned())))
}
