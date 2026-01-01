use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "signup_type", rename_all = "snake_case")]
pub enum SignUpMessage {
    Success {
        active_code: String,
        app_key: String,
        client_email: String,
        email: String,
        language_code: String,
        user_id: Uuid,
    },
}
