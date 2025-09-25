pub struct DbConfig {
    pub db_scheme: Option<String>,
}

pub struct AppConfig {
    pub app_key: String,
    pub db_config: DbConfig,
    pub has_swagger: bool,
    pub has_discovery_service: bool,
}

impl AppConfig {
    pub fn new(
        app_key: String,
        db_scheme: Option<String>,
        has_swagger: bool,
        has_discovery_service: bool,
    ) -> Self {
        Self {
            app_key,
            db_config: DbConfig { db_scheme },
            has_swagger,
            has_discovery_service: has_discovery_service,
        }
    }
}
