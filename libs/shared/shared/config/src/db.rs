use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection};

#[derive(Clone)]
pub struct Database {
    connect_option: ConnectOptions,
    connection: Option<DatabaseConnection>,
}

impl Database {
    pub fn new(db_url_path: Option<String>, scheme: Option<String>) -> Self {
        let database_path = db_url_path.unwrap_or("DATABASE_URL".to_string());
        let con_str = std::env::var(database_path).unwrap();
        let mut opt = ConnectOptions::new(con_str);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8 * 60))
            .sqlx_logging(true)
            // .set_schema_search_path(scheme.unwrap_or("public".to_string()))
            .sqlx_logging_level(log::LevelFilter::Info);

        if let Some(scheme) = scheme {
            opt.set_schema_search_path(scheme);
        }

        Database {
            connection: None,
            connect_option: opt,
        }
    }

    pub async fn connect(&mut self) {
        let connection = sea_orm::Database::connect(self.connect_option.clone())
            .await
            .expect("Could not connect to database");
        self.connection = Some(connection);
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        self.connection.as_ref().unwrap()
    }
}
