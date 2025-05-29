use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    /// Path to Gateway's local directory for storing configuration, interceptors, filters, extensions  and runtime data.
    pub dp: String,

    /// Watch for changes in configuration files, interceptors, filters and extensions and automatically apply updates.
    pub watch: bool,

    /// Reload configuration files and update runtime settings.
    /// May trigger a graceful restart if required.
    pub reload: bool,

    /// Test the server configuration without starting the application.
    pub test: bool,

    /// Display the current version of the API Gateway and exit.
    pub version: bool,

    /// Enable verbose logging for more detailed output.
    /// This is useful for debugging and monitoring.
    pub verbose: bool,

    /// Enable debug mode to output additional debugging information.
    /// Use this to troubleshoot issues during development or runtime.
    pub debug: bool,

    /// Whether this server should try to upgrade from a running old server
    /// It'll work only on linux platforms
    pub upgrade: bool,

    // TODO will remove because it will config in gateway
    // Binding port 0.0.0.0:5000
    pub addr: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dp: String::from("/etc/gateway"),
            watch: false,
            reload: false,
            test: false,
            version: false,
            verbose: false,
            debug: false,
            upgrade: false,
            addr: "0.0.0.0:5000".to_owned(),
        }
    }
}

pub fn load_app_config() -> AppConfig {
    let config = Config::builder()
        .add_source(
            Environment::with_prefix("GATEWAY")
                .try_parsing(true)
                .separator("_"),
        )
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap_or(AppConfig::default());
    tracing::debug!("{:?}", app_config);
    app_config
}
