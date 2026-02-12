use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection};

#[derive(Clone)]
pub struct Database {
    connect_option: ConnectOptions,
    connection: Option<DatabaseConnection>,
    scheme_path: String,
}

impl Database {
    pub fn new(db_url_path: Option<String>, scheme: Option<String>) -> Self {
        let database_path = db_url_path.unwrap_or("DATABASE_URL".to_string());
        let con_str = std::env::var(&database_path)
            .expect(format!("Environment variable '{}' not set", database_path).as_str());
        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or("5".to_string())
            .parse::<u32>()
            .unwrap_or(5);
        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or("100".to_string())
            .parse::<u32>()
            .unwrap_or(100);
        let mut opt = ConnectOptions::new(con_str.clone());
        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8 * 60))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let scheme_path = match scheme {
            Some(s) => s,
            None => {
                panic!("Database schema must be provided");
            }
        };

        opt.set_schema_search_path(scheme_path.clone());

        Database {
            connection: None,
            connect_option: opt,
            scheme_path,
        }
    }

    pub async fn connect(&mut self) {
        let connection = sea_orm::Database::connect(self.connect_option.clone()).await;
        let connection = match connection {
            Ok(conn) => conn,
            Err(e) => {
                let scheme_path = self.scheme_path.clone();
                panic!("Conect to scheme {} . Error: {}  ", scheme_path, e,);
            }
        };
        self.connection = Some(connection);
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        self.connection.as_ref().unwrap()
    }
}
