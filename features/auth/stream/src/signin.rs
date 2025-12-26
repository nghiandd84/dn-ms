use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "signin_type", rename_all = "snake_case")]
pub enum SignInMessage {
    Request {
        user_id: String,
        ip_address: String,
    },
    Success {
        user_id: String,
        ip_address: String,
    },
    Failure {
        user_id: String,
        ip_address: String,
        reason: String,
    },
}
