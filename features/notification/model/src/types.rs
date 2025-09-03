use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};

// Type alias for a client sender channel
pub type ClientSender = mpsc::UnboundedSender<Message>;

// --- Shared WebSocket State ---
pub type Clients = Arc<RwLock<HashMap<usize, ClientSender>>>;

pub fn new_clients() -> Clients {
    Arc::new(RwLock::new(HashMap::new()))
}
