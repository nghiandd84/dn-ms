use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum ServerResponse {
    Auth { status: Auth },
    Payment { platform: String, message: String },
    Notification { message: String },
    Pong,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Auth {
    Success,
    Failure,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum ClientRequest {
    Authenticate { token: String, client_id: Uuid },
    Disconnect,
    Ping,
    LogError { event: String },
}
