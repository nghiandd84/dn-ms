use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum NotificationMessage {
    Notification {
        user_id: Uuid,
        message: String,
    },

    Payment {
        user_id: Uuid,
        platform: String,
        message: String,
    },
}
