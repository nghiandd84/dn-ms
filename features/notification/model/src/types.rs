use axum::extract::ws::Message;
use std::collections::HashMap;
use tokio::sync::mpsc;

// Type alias for a client sender channel
pub type ClientSender = mpsc::UnboundedSender<Message>;

// --- Shared WebSocket State ---
pub type Clients = HashMap<usize, ClientSender>;

pub fn new_clients() -> Clients {
    HashMap::new()
}
