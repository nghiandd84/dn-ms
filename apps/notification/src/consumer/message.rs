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

// {"message_type":"notification", "user_id": "3158787f-7b76-4b04-b79d-4d8fac17d841", "message": "My Message"}
