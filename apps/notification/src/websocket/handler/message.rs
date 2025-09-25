use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::extract::ws::Message::{Close, Text};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use features_email_template_model::types::Clients;
use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use shared_shared_app::state::AppState;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use features_email_template_model::state::{NotificationCacheState, NotificationState};

use crate::websocket::action::client::WebSocketClientAction;
use crate::websocket::handler::authenticate::handle_authenticate;
use crate::websocket::handler::ping::handle_ping;

// Simple counter for unique client IDs
static NEXT_CLIENT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

/// Handler for WebSocket connections.
#[axum::debug_handler]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState<NotificationCacheState, Arc<RwLock<NotificationState>>>>,
) -> impl IntoResponse {
    let notification_state = state.state.unwrap();
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, notification_state))
}

/// Handles a new WebSocket connection.
async fn handle_websocket_connection(
    ws: WebSocket,
    notification_state: Arc<RwLock<NotificationState>>,
) {
    let websocket_id = NEXT_CLIENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // Split the socket into sender and receiver
    let (ws_sender, ws_receiver) = ws.split();
    // Use an MPSC channel to send messages from other tasks to this client's WebSocket

    let (tx, rx) = mpsc::unbounded_channel::<axum::extract::ws::Message>();
    let notification_state_clone = notification_state.clone();

    {
        let mut notification_state_write_guard = notification_state_clone.write().unwrap();
        notification_state_write_guard.insert_client(websocket_id, tx.clone());
        info!("Client {} connected to WebSocket.", websocket_id);
    }

    // Task to send messages from the MPSC channel to the WebSocket
    let send_task = tokio::spawn(handle_send_messages(websocket_id, ws_sender, rx));
    // Task to receive messages from the WebSocket (e.g., pings or close messages)
    let recv_task = tokio::spawn(async move {
        debug!("Client {} starting to receive messages.", websocket_id);
        let mut user_id: Option<String> = None;
        handle_receive_messages(
            websocket_id,
            &mut user_id,
            ws_receiver,
            tx,
            notification_state_clone,
        )
        .await;
    });

    // Wait for either send or receive task to complete (meaning connection closed)
    tokio::select! {
        _ = send_task => {
            // If send task ends, we should also end the receive task
            info!("Send task ended for client {}, closing connection.", websocket_id);

        },
        _ = recv_task => {
            // If receive task ends, we should also end the send task
            info!("Receive task ended for client {}, closing connection.", websocket_id);
        },
    }

    {
        let mut notification_state_write_guard = notification_state.write().unwrap();
        info!("Client {} disconnect to WebSocket.", websocket_id);
        notification_state_write_guard.remove_client(&websocket_id);
    }

    info!("Client {} disconnected.", websocket_id);
}

async fn handle_send_messages(
    client_id: usize,
    mut ws_sender: SplitSink<WebSocket, Message>,
    mut rx: mpsc::UnboundedReceiver<Message>,
) {
    while let Some(message) = rx.recv().await {
        debug!("Client {} sending message: {:?}", client_id, message);
        if let Err(e) = ws_sender.send(message).await {
            warn!("Failed to send message to client {}: {}", client_id, e);
            break;
        }
    }
    info!("Client {} send task finished.", client_id);
}

async fn handle_receive_messages<'a>(
    websocket_id: usize,
    user_id: &mut Option<String>,
    mut ws_receiver: SplitStream<WebSocket>,
    tx: mpsc::UnboundedSender<Message>,
    notification_state: Arc<RwLock<NotificationState>>,
) -> () {
    {
        while let Some(result) = ws_receiver.next().await {
            debug!(
                "Client {} user_id {:?} received message: {:?} ",
                websocket_id, user_id, result
            );
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    warn!("WebSocket receive error for client {}: {}", websocket_id, e);
                    continue;
                }
            };
            match msg {
                Close(_) => {
                    info!("Client {} disconnected.", websocket_id);
                    continue;
                }
                Text(text_msg) => {
                    debug!("Client {} received Text Message {}", websocket_id, text_msg);
                    if user_id.is_none() {
                        warn!(
                            "Client {} is not authenticated, ignoring message.",
                            websocket_id
                        );
                    }
                    // Deserialize the message to WebSocketClientAction
                    let client_action =
                        serde_json::from_str::<WebSocketClientAction>(text_msg.as_str());
                    if let Err(e) = client_action {
                        warn!(
                            "Failed to deserialize message from client {}: {}",
                            websocket_id, e
                        );
                        continue;
                    }
                    let client_action = client_action.unwrap();

                    handle_client_action(client_action, websocket_id, &notification_state, &tx)
                        .await;
                }
                _ => {
                    // If you want to echo the message back, send it through the tx channel
                    if let Err(e) = tx.send(msg) {
                        warn!(
                            "Failed to send message to client {} via channel: {}",
                            websocket_id, e
                        );
                        break;
                    }
                    // Handle other message types if needed
                    debug!("Client {} received non-close message", websocket_id);
                }
            }
        }
        info!(
            "Client {} receive task finished, disconnecting.",
            websocket_id
        );
    }
}

async fn handle_client_action<'a>(
    client_action: WebSocketClientAction,
    websocket_id: usize,
    notification_state: &'a Arc<RwLock<NotificationState>>,
    tx: &mpsc::UnboundedSender<Message>,
) {
    match client_action {
        WebSocketClientAction::Authenticate { token, client_id } => {
            handle_authenticate(token, websocket_id, client_id, notification_state, tx).await;
        }
        WebSocketClientAction::Disconnect => {
            info!("Client {} requested disconnection.", websocket_id);
        }
        WebSocketClientAction::Ping => {
            debug!("Client {} sent a Ping.", websocket_id);
            handle_ping(websocket_id, tx).await;
        }
        _ => {
            info!(
                "Client {} received unsupported action: {:?}",
                websocket_id, client_action
            );
        }
    }
}
