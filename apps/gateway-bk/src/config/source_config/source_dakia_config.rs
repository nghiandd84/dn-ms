use std::{fs, path::Path};

use crate::{
    config::DakiaConfig,
    error::{DakiaError, DakiaResult, ImmutStr},
};

use super::GatewayConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceDakiaRawConfig {
    pub dp: Option<String>,
    pub error_log: Option<String>,
    pub pid_file: Option<String>,
    pub upgrade_sock: Option<String>,
    pub daemon: Option<bool>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub threads: Option<usize>,
    pub work_stealing: Option<bool>,
    pub grace_period_seconds: Option<u64>,
    pub graceful_shutdown_timeout_seconds: Option<u64>,
    pub upstream_keepalive_pool_size: Option<usize>,
    pub upstream_connect_offload_threadpools: Option<usize>,
    pub upstream_connect_offload_thread_per_pool: Option<usize>,
    pub upstream_debug_ssl_keylog: Option<bool>,
    pub gateways: Vec<GatewayConfig>,
}

impl Default for SourceDakiaRawConfig {
    // TODO: keep a yaml embeded string for default config with router and interceptors
    fn default() -> Self {
        SourceDakiaRawConfig {
            dp: None,
            daemon: None,
            error_log: None,
            pid_file: None,
            upgrade_sock: None,
            user: None,
            group: None,
            threads: None,
            work_stealing: None,
            grace_period_seconds: None,
            graceful_shutdown_timeout_seconds: None,
            upstream_connect_offload_thread_per_pool: None,
            upstream_connect_offload_threadpools: None,
            upstream_debug_ssl_keylog: None,
            upstream_keepalive_pool_size: None,
            gateways: vec![],
        }
    }
}

impl From<DakiaConfig> for SourceDakiaRawConfig {
    fn from(dakia_config: DakiaConfig) -> Self {
        Self {
            dp: Some(dakia_config.dp),
            daemon: Some(dakia_config.daemon),
            error_log: Some(dakia_config.error_log),
            pid_file: Some(dakia_config.pid_file),
            upgrade_sock: Some(dakia_config.upgrade_sock),
            user: dakia_config.user,
            group: dakia_config.group,
            threads: Some(dakia_config.threads),
            work_stealing: Some(dakia_config.work_stealing),
            grace_period_seconds: dakia_config.grace_period_seconds,
            graceful_shutdown_timeout_seconds: dakia_config.graceful_shutdown_timeout_seconds,
            upstream_connect_offload_thread_per_pool: dakia_config
                .upstream_connect_offload_thread_per_pool,
            upstream_connect_offload_threadpools: dakia_config.upstream_connect_offload_threadpools,
            upstream_debug_ssl_keylog: Some(dakia_config.upstream_debug_ssl_keylog),
            upstream_keepalive_pool_size: Some(dakia_config.upstream_keepalive_pool_size),
            gateways: dakia_config.gateways,
        }
    }
}

impl SourceDakiaRawConfig {
    pub fn flush(&self) -> DakiaResult<()> {
        let string_config = serde_yaml::to_string(self).map_err(|e| {
            DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(ImmutStr::from("Faild to flush dakia config to file")),
                Some(Box::new(e)),
            )
        })?;

        let dp = self.dp.as_deref().unwrap_or("/etc/dakia"); // dakia path
        let cp = Path::new(dp).join("config/dakia.yaml"); // configs path

        fs::write(cp, string_config).map_err(|e| {
            DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(ImmutStr::from("Faild to flush dakia config to file")),
                Some(Box::new(e)),
            )
        })?;

        Ok(())
    }
}
