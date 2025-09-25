use axum::extract::ws::Message;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{debug, error};
use uuid::Uuid;

use features_email_template_model::state::NotificationState;

use crate::websocket::action::server::{Auth, WebSocketServerResponse};

pub async fn handle_authenticate<'a>(
    token: String,
    websocket_id: usize,
    client_id: Uuid,
    notification_state: &'a Arc<RwLock<NotificationState>>,
    tx: &'a mpsc::UnboundedSender<Message>,
) {
    let auth_server = std::env::var("AUTH_SERVER").expect("AUTH_SERVER must be set");
    let verify_endpoint = std::env::var("AUTH_ENDPOINT_VERIFY_TOKEN")
        .expect("AUTH_ENDPOINT_VERIFY_TOKEN must be set");

    let client = reqwest::Client::new();
    let url = format!("{}{}", auth_server, verify_endpoint);
    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "token": token,  "client_id": client_id }))
        .send()
        .await;
    // Check res status is 200 and get body
    debug!("Response from auth server: {res:#?}");

    if res.is_err() {
        let err_msg = format!(
            "Failed to send request to auth server: {}",
            res.err().unwrap()
        );
        send_failure_message(tx, err_msg);
        return;
    }
    let res = res.unwrap();
    if !res.status().is_success() {
        let err_msg = format!("Authentication failed with status: {}", res.status());
        send_failure_message(tx, err_msg);
        return;
    }
    let body = res.text().await;
    if body.is_err() {
        let err_msg = format!("Failed to read response body: {}", body.err().unwrap());
        send_failure_message(tx, err_msg);
        return;
    }
    let body = body.unwrap();
    let data = serde_json::from_str::<serde_json::Value>(&body);
    if data.is_err() {
        let err_msg = format!("Failed to parse response body: {}", data.err().unwrap());

        send_failure_message(tx, err_msg);
        return;
    }
    let data = data.unwrap();
    let data = data.get("data").unwrap();
    debug!("Parsed response body: {:#?}", data);
    if data.get("user_id").is_none() {
        let err_msg = "Response body does not contain user_id".to_string();
        send_failure_message(tx, err_msg);
        return;
    }
    let user_id = data.get("user_id").unwrap().as_str();
    if user_id.is_none() {
        let err_msg = "user_id is not a string".to_string();
        send_failure_message(tx, err_msg);

        return;
    }
    let user_id = user_id.unwrap();
    let user_id = Uuid::parse_str(user_id);
    if user_id.is_err() {
        let err_msg = format!("Failed to parse user_id: {}", user_id.err().unwrap());
        send_failure_message(tx, err_msg);
        return;
    }
    let user_id = user_id.unwrap();
    debug!("Mapped user_id {} to client_id {}", user_id, websocket_id);
    {
        let mut state_read_guard = notification_state.write().unwrap();
        state_read_guard.insert_user_client_mapping(user_id, websocket_id);
    }

    let websocket_message = WebSocketServerResponse::Auth {
        status: Auth::Success,
    };
    if let Err(e) = tx.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        error!("Failed to send message to user {:?}: {}", user_id, e);
    }
}

fn send_failure_message(tx: &mpsc::UnboundedSender<Message>, err_msg: String) {
    error!("Authentication failed: {}", err_msg);
    let websocket_message = WebSocketServerResponse::Auth {
        status: Auth::Failure,
    };
    if let Err(e) = tx.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        error!("Failed to send failure message: {}", e);
    }
}
