use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WebSocketServerResponse {
    AuthSuccess { user_id: String },
    AuthFailure { error: String },
    // NotificationSuccess(NotificationSuccess),
    // NotificationFailure(NotificationFailure),
    // KafkaStatusUpdate(KafkaStatusUpdate),
    Pong, // Added Pong response from server
}
