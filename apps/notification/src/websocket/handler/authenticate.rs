use axum::extract::ws::Message;
use tokio::sync::mpsc;
use tracing::debug;

use shared_shared_app::state::AppState;

use features_email_template_model::state::{NotificationCacheState, NotificationState};
use uuid::Uuid;

pub async fn handle_authenticate<'a>(
    token: String,
    websocket_id: usize,
    client_id: Uuid,
    app_state: &'a AppState<NotificationCacheState, NotificationState>,
    tx: &'a mpsc::UnboundedSender<Message>,
) {
    let state = app_state.state.as_ref().unwrap();
    let clients = state.get_clients();
    let len = clients.read().await.len();
    debug!(
        "Client {} authenticated with token: {}",
        websocket_id, token
    );
    debug!("Number of connected clients: {}", len);

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
        debug!("{}", err_msg);
        let _ = tx.send(Message::Text(err_msg.into()));
        return;
    }
    let res = res.unwrap();
    if !res.status().is_success() {
        let err_msg = format!("Authentication failed with status: {}", res.status());
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let body = res.text().await;
    if body.is_err() {
        let err_msg = format!("Failed to read response body: {}", body.err().unwrap());
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let body = body.unwrap();
    debug!("Response body: {}", body);
    let data = serde_json::from_str::<serde_json::Value>(&body);
    if data.is_err() {
        let err_msg = format!("Failed to parse response body: {}", data.err().unwrap());
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let data = data.unwrap();
    let data = data.get("data").unwrap();
    debug!("Parsed response body: {:#?}", data);
    if data.get("user_id").is_none() {
        let err_msg = "Response body does not contain user_id".to_string();
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let user_id = data.get("user_id").unwrap().as_str();
    if user_id.is_none() {
        let err_msg = "user_id is not a string".to_string();
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let user_id = user_id.unwrap();
    let user_id = Uuid::parse_str(user_id);
    if user_id.is_err() {
        let err_msg = format!("Failed to parse user_id: {}", user_id.err().unwrap());
        debug!("{}", err_msg);
        // let _ = tx.send(Message::Text(err_msg));
        return;
    }
    let user_id = user_id.unwrap();
    // Map user_id to client_id
    // state_one.insert_user_client_mapping(user_id, websocket_id);
    debug!("Mapped user_id {} to client_id {}", user_id, websocket_id);
    // Send a authentication success message back to client
}
