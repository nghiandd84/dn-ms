use serde_json::{de, json};
use uuid::Uuid;

use shared_shared_macro::RemoteService;
use shared_shared_middleware::RequestTracingMiddleware;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct TokenService {}

impl TokenService {
    pub async fn validate_token(token: String) -> Result<Uuid, String> {
        let body = json!({
            "token": token
        });

        let verify_endpoint = std::env::var("AUTH_ENDPOINT_VERIFY_TOKEN")
            .expect("AUTH_ENDPOINT_VERIFY_TOKEN must be set");

        let res = Self::call_api(verify_endpoint, Method::POST, Some(body), HashMap::new()).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data = res.unwrap();
        debug!("Token validation response: {:?}", data);

        if data.get("user_id").is_none() {
            let err_msg = "Response body does not contain user_id".to_string();
            return Err(err_msg);
        }
        let user_id = data.get("user_id").unwrap().as_str();
        if user_id.is_none() {
            let err_msg = "user_id is not a string".to_string();
            return Err(err_msg);
        }
        let user_id = user_id.unwrap();
        let user_id = Uuid::parse_str(user_id);
        if user_id.is_err() {
            let err_msg = format!("Failed to parse user_id: {}", user_id.err().unwrap());
            return Err(err_msg);
        }
        let user_id = user_id.unwrap();
        Ok(user_id)
    }
}
