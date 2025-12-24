use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum SignUpMessage {
    Success { user_id: Uuid, email: String },
}
