use axum::extract::ws::Message::{Close, Text};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use shared_shared_app::state::AppState;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};
use tracing_subscriber::field::debug;

use crate::{app::NotificationCacheState, websocket::action::client::WebSocketClientAction};

// Simple counter for unique client IDs
static NEXT_CLIENT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

/// Handler for WebSocket connections.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState<NotificationCacheState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, state))
}

/// Handles a new WebSocket connection.
async fn handle_websocket_connection(ws: WebSocket, state: AppState<NotificationCacheState>) {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    info!("Client {} connected to WebSocket.", client_id);

    // Split the socket into sender and receiver
    let (ws_sender, ws_receiver) = ws.split();
    // Use an MPSC channel to send messages from other tasks to this client's WebSocket
    let (tx, rx) = mpsc::unbounded_channel::<axum::extract::ws::Message>();

    // Task to send messages from the MPSC channel to the WebSocket
    let send_task = tokio::spawn(handle_send_messages(client_id, ws_sender, rx));
    // Task to receive messages from the WebSocket (e.g., pings or close messages)
    let recv_task = tokio::spawn(async move {
        debug!("Client {} starting to receive messages.", client_id);
        let mut user_id: Option<String> = None;
        handle_receive_messages(client_id, &mut user_id, ws_receiver, tx).await;
    });

    // Wait for either send or receive task to complete (meaning connection closed)
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    info!("Client {} disconnected.", client_id);
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
    client_id: usize,
    mut user_id: &mut Option<String>,
    mut ws_receiver: SplitStream<WebSocket>,
    tx: mpsc::UnboundedSender<Message>,
) {
    {
        while let Some(result) = ws_receiver.next().await {
            debug!(
                "Client {} user_id {:?} received message: {:?} ",
                client_id, user_id, result
            );
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    warn!("WebSocket receive error for client {}: {}", client_id, e);
                    break;
                }
            };
            match msg {
                Close(_) => {
                    info!("Client {} disconnected.", client_id);
                    break;
                }
                Text(text_msg) => {
                    debug!("Client {} received Text Message {}", client_id, text_msg);
                    if user_id.is_none() {
                        warn!(
                            "Client {} is not authenticated, ignoring message.",
                            client_id
                        );
                    }
                    // Deserialize the message to WebSocketClientAction
                    let client_action =
                        serde_json::from_str::<WebSocketClientAction>(text_msg.as_str());
                    if let Err(e) = client_action {
                        warn!(
                            "Failed to deserialize message from client {}: {}",
                            client_id, e
                        );
                        continue;
                    }
                    let client_action = client_action.unwrap();

                    // Handle the client action
                    match client_action {
                        WebSocketClientAction::Authenticate { token } => {
                            debug!("Client {} authenticated with token: {}", client_id, token);
                            // Here you would typically validate the token and set the user_id
                            // For demonstration, we'll just set a dummy user_id
                            *user_id = Some("MyUserId".to_string());
                            debug!(
                                "Client {} token: {:?} and new user_id: {}",
                                client_id,
                                token,
                                user_id.as_deref().unwrap_or("None")
                            );
                        }
                        _ => {
                            debug!(
                                "Client {} received unsupported action: {:?}",
                                client_id, client_action
                            );
                            // Handle other actions if needed
                        }
                    }
                    // user_id = &x;
                }
                _ => {
                    // If you want to echo the message back, send it through the tx channel
                    if let Err(e) = tx.send(msg) {
                        warn!(
                            "Failed to send message to client {} via channel: {}",
                            client_id, e
                        );
                        break;
                    }
                    // Handle other message types if needed
                    debug!("Client {} received non-close message", client_id);

                    // }
                }
            }
        }
        info!("Client {} receive task finished, disconnecting.", client_id);
    }
}
