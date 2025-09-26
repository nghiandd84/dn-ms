use rs_consul::Consul;
use std::sync::{LazyLock, Mutex};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::roundrobin::RoundRobin;
use shared_shared_macro::RemoteService;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct TokenService {}

impl TokenService {
    pub async fn validate_token<F>(
        token: String,
        client_id: Uuid,
        on_api_failed: F,
    ) -> Result<Uuid, String>
    where
        F: Fn(String) + Send + Sync,
    {
        let data: (String, u16) = Self::get_instance().unwrap();
        let (ip, port) = data;
        let auth_server = format!("http://{}:{}", ip, port);
        let verify_endpoint = std::env::var("AUTH_ENDPOINT_VERIFY_TOKEN")
            .expect("AUTH_ENDPOINT_VERIFY_TOKEN must be set");
        let client = reqwest::Client::new();
        let url = format!("{}{}", auth_server, verify_endpoint);
        debug!("Verifying token at URL: {}", url);
        let res = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "token": token,  "client_id": client_id }))
            .send()
            .await;
        debug!("Response from auth server: {res:#?}");

        if let Err(e) = res {
            let err_msg = format!("Failed to send request to auth server: {}", e);
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }

        let res = res.unwrap();
        if !res.status().is_success() {
            let err_msg = format!("Authentication failed with status: {}", res.status());
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let body = res.text().await;
        if body.is_err() {
            let err_msg = format!("Failed to read response body: {}", body.err().unwrap());
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let body = body.unwrap();
        let data = serde_json::from_str::<serde_json::Value>(&body);
        if data.is_err() {
            let err_msg = format!("Failed to parse response body: {}", data.err().unwrap());
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let data = data.unwrap();
        let data = data.get("data").unwrap();
        debug!("Parsed response body: {:#?}", data);
        if data.get("user_id").is_none() {
            let err_msg = "Response body does not contain user_id".to_string();
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let user_id = data.get("user_id").unwrap().as_str();
        if user_id.is_none() {
            let err_msg = "user_id is not a string".to_string();
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let user_id = user_id.unwrap();
        let user_id = Uuid::parse_str(user_id);
        if user_id.is_err() {
            let err_msg = format!("Failed to parse user_id: {}", user_id.err().unwrap());
            on_api_failed(err_msg.clone());
            return Err(err_msg);
        }
        let user_id = user_id.unwrap();
        Ok(user_id)
    }
}
