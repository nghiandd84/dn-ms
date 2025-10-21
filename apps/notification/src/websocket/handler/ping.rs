use axum::extract::ws::Message;
use tokio::sync::mpsc;
use tracing::{debug, error};

use features_notification_stream::websocket::ServerResponse;

pub async fn handle_ping<'a>(client_id: usize, tx: &'a mpsc::UnboundedSender<Message>) {
    debug!("Client {} PING", client_id);
    let pong_msg = ServerResponse::Pong;
    if let Err(e) = tx.send(Message::Text(
        serde_json::to_string(&pong_msg).unwrap().into(),
    )) {
        error!("Failed to send pong to client {}: {}", client_id, e);
    }
}
