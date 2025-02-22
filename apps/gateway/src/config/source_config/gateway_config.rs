use crate::error::BError;
use crate::error::DakiaError;
use crate::error::DakiaResult;
use crate::gateway::filter::exec_filter;
use crate::proxy::http::Session;
use crate::qe::query::Query;

use super::interceptor_config::InterceptorConfig;
use super::DownstreamConfig;
use super::InetAddress;
use super::RouterConfig;
use super::UpstreamConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    pub name: String, // TODO: use auto generated name
    // TODO: add type = HTTP, TCP, SMTP, etc
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,

    #[serde(default)]
    pub routers: Vec<RouterConfig>,

    #[serde(default)]
    pub interceptors: Vec<InterceptorConfig>,

    #[serde(default)]
    pub filters: Vec<Query>,
}

pub fn find_router_config<'a>(session: &'a Session<'a>) -> DakiaResult<Option<&'a RouterConfig>> {
    for router_config in session.ctx().gateway_state.gateway_config().routers.iter() {
        match &router_config.filter {
            None => return Ok(Some(router_config)), // if no filter present for any router then it'll be considered a match when encountered
            Some(filter_name) => {
                let filter = session.ctx().gateway_state.filter_or_err(&filter_name)?;
                let is_matched = exec_filter(filter, session)?;
                if is_matched {
                    return Ok(Some(router_config));
                }
            }
        }
    }
    Ok(None)
}

pub fn find_router_config_or_err<'a>(session: &'a Session<'a>) -> DakiaResult<&'a RouterConfig> {
    let router_config = find_router_config(session)?;
    router_config.ok_or(DakiaError::create_unknown_context(
        crate::error::ImmutStr::Static("router config not found".into()),
    ))
}

impl GatewayConfig {
    pub fn find_default_upstream(&self) -> Option<&UpstreamConfig> {
        self.upstreams
            .iter()
            .find(|upstream_config| upstream_config.default)
    }

    pub fn find_upstream_config(
        &self,
        upstream_name: &str,
        fallback: bool,
    ) -> Option<&UpstreamConfig> {
        let upstream_config = self
            .upstreams
            .iter()
            .find(|upstream_config| upstream_config.name == upstream_name);
        match upstream_config {
            Some(upstream_config) => Some(upstream_config),
            None => {
                if fallback {
                    self.find_default_upstream()
                } else {
                    None
                }
            }
        }
    }

    pub fn find_upstream_config_or_err(
        &self,
        upstream_name: &str,
        fallback: bool,
    ) -> Result<&UpstreamConfig, BError> {
        self.find_upstream_config(upstream_name, fallback).ok_or(
            DakiaError::create_unknown_context(crate::error::ImmutStr::Static(
                "upstream config not found".into(),
            )),
        )
    }
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            bind_addresses: Default::default(),
            downstreams: Default::default(),
            upstreams: Default::default(),
            routers: Default::default(),
            interceptors: Default::default(),
            filters: Default::default(),
        }
    }
}
