use rs_consul::Consul;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_macro::RemoteService;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct AuthenticationRequestService {}

impl AuthenticationRequestService {
    pub async fn authenticate_request(
        client_id: Uuid,
        scope: Vec<String>,
        redirect_uri: String,
        response_type: String,
        state: String,
    ) -> Result<Uuid, String> {
        let body = serde_json::json!({
            "client_id": client_id,
            "scopes": scope,
            "redirect_uri": redirect_uri,
            "response_type": response_type,
            "state": state
        });
        let auth_endpoint = std::env::var("AUTH_ENDPOINT_AUTHENTICATE_REQUEST")
            .expect("AUTH_ENDPOINT_AUTHENTICATE_REQUEST must be set");

        let res = Self::call_api(auth_endpoint, reqwest::Method::POST, body, HashMap::new()).await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data = res.unwrap();

        if data.get("id").is_none() {
            let err_msg = "Response body does not contain id".to_string();
            return Err(err_msg);
        }
        let request_id = data.get("id").unwrap().as_str();
        if request_id.is_none() {
            let err_msg = "request_id is not a string".to_string();
            return Err(err_msg);
        }
        let request_id = request_id.unwrap();
        let request_id = Uuid::parse_str(request_id);
        if request_id.is_err() {
            let err_msg = format!("Failed to parse request_id: {}", request_id.err().unwrap());
            return Err(err_msg);
        }
        let request_id = request_id.unwrap();
        Ok(request_id)
    }

    pub async fn login_password(
        email: String,
        password: String,
        state: Uuid,
    ) -> Result<String, String> {
        let body = serde_json::json!({
            "email": email,
            "password": password,
            "state": state
        });

        let login_password_endpoint = std::env::var("AUTH_ENDPOINT_LOGIN_PASSWORD")
            .expect("AUTH_ENDPOINT_LOGIN_PASSWORD must be set");

        let res = Self::call_api(
            login_password_endpoint,
            reqwest::Method::POST,
            body,
            HashMap::new(),
        )
        .await;
        if res.is_err() {
            let err_msg = res.err().unwrap();
            return Err(err_msg);
        }
        let data = res.unwrap();

        let code = data.get("code").unwrap().as_str();
        if code.is_none() {
            let err_msg = "Response do not contains code".to_string();
            return Err(err_msg);
        }
        let code = code.unwrap();

        Ok(code.to_string())
    }
}
