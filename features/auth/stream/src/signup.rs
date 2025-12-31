use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "signup_type", rename_all = "snake_case")]
pub enum SignUpMessage {
    Success {
        user_id: Uuid,
        email: String,
        app_key: String,
        language_code: String,
        active_code: String,
    },
}
