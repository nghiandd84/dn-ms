use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum WebSocketClientAction {
    Authenticate { token: String, client_id: Uuid },
    Disconnect,
    Ping,
}
