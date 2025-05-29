use config::{Config, File, FileFormat};
use pingora::{prelude::Opt, server::configuration::ServerConf};
use serde::Deserialize;
use std::{fs, path::Path};
use tracing::debug;

use super::{app_config::AppConfig, source_config::GatewayConfig};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DnConfig {
    #[serde(skip_deserializing)]
    pub version: i64,
    pub daemon: bool,
    #[serde(skip_deserializing)]
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

impl DnConfig {
    pub fn from_args(app_config: &AppConfig) -> Self {
        let directory_path = app_config.dp.as_str();
        let config_path = Path::new(directory_path).join("config/dn-config.yaml");
        let raw_config = fs::read_to_string(&config_path).unwrap();
        let config = Config::builder()
            .add_source(File::from_str(raw_config.as_str(), FileFormat::Yaml))
            .build()
            .unwrap();
        let mut dn_config: DnConfig = config.try_deserialize().unwrap();
        dn_config.version = 0;
        dn_config.dp = app_config.dp.clone();
        debug!("\n{:#?}", dn_config);
        dn_config
    }

    pub fn to_pingore_opt(&self, args: &AppConfig) -> Opt {
        let mut opt = Opt::default();
        opt.daemon = self.daemon;
        opt.upgrade = args.upgrade;

        opt
    }
}

impl Into<ServerConf> for DnConfig {
    fn into(self) -> ServerConf {
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
