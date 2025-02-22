use redis::{aio::Connection, Client};
use std::env;

#[derive(Clone)]
pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new(cache_url_path: Option<String>) -> Self {
        let redis_path = cache_url_path.unwrap_or("REDIS_URL".to_string());
        let url = env::var(redis_path).unwrap();
        let client = Client::open(url).unwrap();

        Self { client }
    }

    pub async fn get_connection(&self) -> Result<Connection, String> {
        let con = self.client.get_tokio_connection().await;

        match con {
            Ok(con) => Ok(con),
            Err(err) => Err(err.to_string()),
        }
    }
}
