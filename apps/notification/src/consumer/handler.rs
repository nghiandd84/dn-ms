use axum::extract::ws::Message;
use std::sync::{Arc, RwLock};
use tracing::{debug, error};

use features_email_template_model::state::NotificationState;

use crate::{consumer::event::KafkaEvent, websocket::action::server::WebSocketServerResponse};

pub async fn handler_event(event: KafkaEvent, notification_state: Arc<RwLock<NotificationState>>) {
    match event {
        KafkaEvent::Notification { user_id, message } => {
            handle_notification_event(notification_state, user_id, message).await;
        }
        KafkaEvent::Payment {
            user_id,
            platform,
            message,
        } => {
            handle_payment_event(notification_state, user_id, platform, message).await;
        }
    }
}

async fn handle_payment_event<'a>(
    notification_state: Arc<RwLock<NotificationState>>,
    user_id: uuid::Uuid,
    platform: String,
    message: String,
) {
    let client_sender = {
        let state_read_guard = notification_state.write().unwrap();
        let client_sender = state_read_guard.get_client_sender_by_user_id(user_id);
        client_sender
    };
    if client_sender.is_none() {
        debug!("No client sender found for user_id {:?}", user_id);
        return;
    }
    let client_sender = client_sender.unwrap();
    let websocket_message = WebSocketServerResponse::Payment { platform, message };
    if let Err(e) = client_sender.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        debug!("Failed to send message to user {:?}: {}", user_id, e);
    }
}

async fn handle_notification_event<'a>(
    notification_state: Arc<RwLock<NotificationState>>,
    user_id: uuid::Uuid,
    message: String,
) {
    let client_sender = {
        let state_read_guard = notification_state.write().unwrap();
        let client_sender = state_read_guard.get_client_sender_by_user_id(user_id);
        client_sender
    };
    if client_sender.is_none() {
        debug!("No client sender found for user_id {:?}", user_id);
        return;
    }
    let client_sender = client_sender.unwrap();
    let websocket_message = WebSocketServerResponse::Notification { message };
    if let Err(e) = client_sender.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        error!("Failed to send message to user {:?}: {}", user_id, e);
    }
}
