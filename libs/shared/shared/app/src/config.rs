
pub struct DbConfig {
    pub db_scheme: Option<String>,
}

pub struct AppConfig {
    pub app_key: String,
    pub db_config: DbConfig,
}