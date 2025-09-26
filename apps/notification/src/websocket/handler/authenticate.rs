use axum::extract::ws::Message;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{debug, error};
use uuid::Uuid;

use features_auth_remote::TokenService;
use features_email_template_model::state::NotificationState;

use crate::websocket::action::server::{Auth, WebSocketServerResponse};

pub async fn handle_authenticate<'a>(
    token: String,
    websocket_id: usize,
    client_id: Uuid,
    notification_state: &'a Arc<RwLock<NotificationState>>,
    tx: &'a mpsc::UnboundedSender<Message>,
) {
    let validate_token = TokenService::validate_token(token, client_id, |err_msg| {
        send_failure_message(tx, err_msg);
    })
    .await
    .unwrap();

    let user_id = validate_token;
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
