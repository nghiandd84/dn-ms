use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "event_type", rename_all = "camelCase")]
pub enum KafkaEvent {
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
