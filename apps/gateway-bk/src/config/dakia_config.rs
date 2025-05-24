use std::{fs, path::Path};

use tracing::debug;
use pingora::{prelude::Opt, server::configuration::ServerConf};

use crate::{
    config::source_config::SourceDakiaRawConfig,
    error::{DakiaError, DakiaResult, ImmutStr},
    shared::into::IntoRef,
};

use super::{source_config::GatewayConfig, DakiaArgs};

pub type ConfigVersion = i64;

#[derive(Debug, Clone)]
pub struct DakiaConfig {
    pub version: ConfigVersion,
    pub daemon: bool,
    pub dp: String,
    pub error_log: String,
    pub pid_file: String,
    pub upgrade_sock: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub threads: usize,
    pub work_stealing: bool,
    pub grace_period_seconds: Option<u64>,
    pub graceful_shutdown_timeout_seconds: Option<u64>,
    pub upstream_keepalive_pool_size: usize,
    pub upstream_connect_offload_threadpools: Option<usize>,
    pub upstream_connect_offload_thread_per_pool: Option<usize>,
    pub upstream_debug_ssl_keylog: bool,
    pub gateways: Vec<GatewayConfig>,
}

impl Default for DakiaConfig {
    fn default() -> Self {
        Self {
            version: 0,
            dp: Default::default(),
            daemon: Default::default(),
            error_log: Default::default(),
            pid_file: Default::default(),
            upgrade_sock: Default::default(),
            user: Default::default(),
            group: Default::default(),
            threads: Default::default(),
            work_stealing: Default::default(),
            grace_period_seconds: Default::default(),
            graceful_shutdown_timeout_seconds: Default::default(),
            upstream_keepalive_pool_size: Default::default(),
            upstream_connect_offload_threadpools: Default::default(),
            upstream_connect_offload_thread_per_pool: Default::default(),
            upstream_debug_ssl_keylog: Default::default(),
            gateways: Default::default(),
        }
    }
}

impl DakiaConfig {
    pub fn from_args(args: DakiaArgs) -> DakiaResult<Self> {
        let dp = args.dp.as_deref().unwrap_or("/etc/dakia"); // dakia path
        let cp = Path::new(dp).join("config/dakia.yaml"); // configs path
        debug!("Config Path {:?}", cp);
        let is_config_file_readable = fs::metadata(&cp)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);

        debug!("is_config_file_readable {}", is_config_file_readable);

        if !is_config_file_readable {
            let e = DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(ImmutStr::from("Failed to load Dakia config file. The file might be missing, inaccessible, or malformed!")),
                None,
            );
            return Err(e);
        }

        let raw_config = fs::read_to_string(&cp).map_err(|e| DakiaError::create(
            crate::error::ErrorType::InternalError,
            crate::error::ErrorSource::Internal,
            Some(ImmutStr::from("Failed to load Dakia config file. The file might be missing, inaccessible, or malformed!")),
            Some(Box::new(e)),
        ))?;
        // debug!("Raw config {:?}", raw_config);
        let mut source_dakia_config: SourceDakiaRawConfig = serde_yaml::from_str(&raw_config)
            .map_err(|e| {
                DakiaError::create(
                    crate::error::ErrorType::InternalError,
                    crate::error::ErrorSource::Internal,
                    Some(ImmutStr::from("Failed to parse config the file.")),
                    Some(Box::new(e)),
                )
            })?;

        // update this so that it can be preserved over restart
        source_dakia_config.dp = args.dp;
        /*
        debug!(
            "\n========== Dakia Config ==========\n{:#?}\n===================================",
            source_dakia_config
        );
         */

        Ok(DakiaConfig::from(source_dakia_config))
    }
    pub fn find_gateway_config<'a>(&'a self, gateway_name: &str) -> Option<&'a GatewayConfig> {
        self.gateways.iter().find(|g| g.name == gateway_name)
    }
    pub fn find_gateway_config_or_err(&self, gateway_name: &str) -> DakiaResult<&GatewayConfig> {
        let gateway_config =
            self.find_gateway_config(gateway_name)
                .ok_or(DakiaError::create_unknown_context(ImmutStr::Static(
                    "gateway config not found".into(),
                )))?;
        Ok(gateway_config)
    }
    pub fn to_pingore_opt(&self, args: &DakiaArgs) -> Opt {
        let mut opt = Opt::default();
        opt.daemon = self.daemon;
        opt.upgrade = args.upgrade;
        // not required, as we are pssing struct directly
        // opt.conf = Some(self.dp.clone() + "/config/pingora.yaml");
        opt
    }
}

impl From<SourceDakiaRawConfig> for DakiaConfig {
    fn from(source_dakia_raw_config: SourceDakiaRawConfig) -> Self {
        DakiaConfig {
            version: 0,
            daemon: source_dakia_raw_config.daemon.unwrap_or(false),
            dp: source_dakia_raw_config
                .dp
                .unwrap_or("/etc/dakia".to_string()),
            error_log: source_dakia_raw_config
                .error_log
                .unwrap_or("/var/log/dakia/error.log".to_string()),
            pid_file: source_dakia_raw_config
                .pid_file
                .unwrap_or("/tmp/dakia.pid".to_string()),
            upgrade_sock: source_dakia_raw_config
                .upgrade_sock
                .unwrap_or("/tmp/dakia_upgrade.sock".to_string()),
            user: source_dakia_raw_config.user.clone(),
            group: source_dakia_raw_config.group.clone(),
            threads: source_dakia_raw_config.threads.unwrap_or(1),
            work_stealing: source_dakia_raw_config.work_stealing.unwrap_or(true),
            grace_period_seconds: source_dakia_raw_config.grace_period_seconds,
            graceful_shutdown_timeout_seconds: source_dakia_raw_config
                .graceful_shutdown_timeout_seconds,
            upstream_keepalive_pool_size: source_dakia_raw_config
                .upstream_keepalive_pool_size
                .unwrap_or(128),
            upstream_connect_offload_threadpools: source_dakia_raw_config
                .upstream_connect_offload_threadpools,
            upstream_connect_offload_thread_per_pool: source_dakia_raw_config
                .upstream_connect_offload_thread_per_pool,
            upstream_debug_ssl_keylog: source_dakia_raw_config
                .upstream_debug_ssl_keylog
                .unwrap_or(false),
            gateways: source_dakia_raw_config.gateways,
        }
    }
}

impl IntoRef<ServerConf> for DakiaConfig {
    fn into_ref(&self) -> ServerConf {
        ServerConf {
            listener_tasks_per_fd: 1,
            max_retries: 5,

            daemon: self.daemon,
            error_log: Some(self.error_log.clone()),
            grace_period_seconds: self.grace_period_seconds,
            graceful_shutdown_timeout_seconds: self.graceful_shutdown_timeout_seconds,
            group: self.group.clone(),
            user: self.user.clone(),
            threads: self.threads,
            pid_file: self.pid_file.clone(),
            upgrade_sock: self.upgrade_sock.clone(),
            upstream_connect_offload_thread_per_pool: self.upstream_connect_offload_thread_per_pool,
            upstream_debug_ssl_keylog: self.upstream_debug_ssl_keylog,
            upstream_connect_offload_threadpools: self.upstream_connect_offload_threadpools,
            upstream_keepalive_pool_size: self.upstream_keepalive_pool_size,
            work_stealing: self.work_stealing,
            version: 1,
            ca_file: None,
            client_bind_to_ipv4: vec![],
            client_bind_to_ipv6: vec![],
        }
    }
}
