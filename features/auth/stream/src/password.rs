use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "password_type", rename_all = "snake_case")]
pub enum PasswordMessage {
    ChangeRequest {
        user_id: String,
        email: String,
        change_code: String,
        language_code: String,
    },
    ResetRequest {
        user_id: String,
        email: String,
        reset_code: String,
        language_code: String,
    },
    Changed {
        user_id: String,
        email: String,
    },
}
