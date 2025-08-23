use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum WebSocketClientAction {
    Authenticate { token: String },
    Disconnect,
    // Add other client actions here if needed
}
