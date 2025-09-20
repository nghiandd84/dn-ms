use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum WebSocketServerResponse {
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
