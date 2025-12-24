use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "message_type", rename_all = "snake_case")]
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
