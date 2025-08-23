// src/main.rs
use anyhow::{anyhow, Result};
use futures_util::{FutureExt, StreamExt, SinkExt};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Headers, Message};
use rdkafka::OffsetReset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::AsyncMailer;
use lettre::Message as EmailMessage;
use serde_json::json; // Import the json! macro

// Axum specific imports
use axum::{
    extract::{
        ws::{Message as AxumWsMessage, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse, Response},
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;


// --- Configuration Structs ---
#[derive(Debug, Deserialize)]
struct KafkaEvent {
    email: String,
    template_id: String,
    data: HashMap<String, String>, // Dynamic data for template placeholders
}

#[derive(Debug, Deserialize)]
struct EmailTemplate {
    subject: String,
    body: String,
}

#[derive(Debug, Serialize)]
struct NotificationSuccess {
    message: String,
    recipient: String,
    template_id: String,
    timestamp: u64, // Unix timestamp
}

#[derive(Debug, Serialize)]
struct NotificationFailure {
    message: String,
    recipient: Option<String>,
    template_id: Option<String>,
    timestamp: u64, // Unix timestamp
    error: String,
}

#[derive(Debug, Serialize)]
struct KafkaStatusUpdate {
    status: String,
    message: String,
    error: Option<String>,
}

// --- New Structs for WebSocket Authentication Message ---

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
enum WebSocketClientAction {
    Authenticate { token: String },
    // Add other client actions here if needed
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum WebSocketServerResponse {
    AuthSuccess { user_id: String },
    AuthFailure { error: String },
    NotificationSuccess(NotificationSuccess),
    NotificationFailure(NotificationFailure),
    KafkaStatusUpdate(KafkaStatusUpdate),
}

// --- Shared WebSocket State ---
// A map of connected client IDs to their WebSocket senders.
// We no longer store user_id or auth status directly in the map,
// as that state is managed within the `handle_websocket_connection` task
// for simplicity given the current broadcast nature of notifications.
type Clients = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<AxumWsMessage>>>>;

// Simple counter for unique client IDs
static NEXT_CLIENT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

/// Fetches an email template from an external API.
async fn fetch_email_template(template_id: &str) -> Result<EmailTemplate> {
    let api_url = std::env::var("EMAIL_TEMPLATE_API_URL")
        .map_err(|_| anyhow!("EMAIL_TEMPLATE_API_URL not set"))?;
    let url = format!("{}/{}", api_url, template_id);
    log::info!("Fetching template from: {}", url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let response = client.get(&url).send().await?.error_for_status()?;
    let template_data: EmailTemplate = response.json().await?;
    log::info!("Successfully fetched template '{}'.", template_id);
    Ok(template_data)
}

/// Sends an email using the configured SMTP server.
async fn send_email(to_email: &str, subject: &str, body: &str) -> Result<()> {
    let smtp_server = std::env::var("SMTP_SERVER").map_err(|_| anyhow!("SMTP_SERVER not set"))?;
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .map_err(|_| anyhow!("SMTP_PORT not set"))?
        .parse()?;
    let smtp_username =
        std::env::var("SMTP_USERNAME").map_err(|_| anyhow!("SMTP_USERNAME not set"))?;
    let smtp_password =
        std::env::var("SMTP_PASSWORD").map_err(|_| anyhow!("SMTP_PASSWORD not set"))?;

    let email = EmailMessage::builder()
        .from(smtp_username.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::parse("text/html; charset=utf-8").map_err(|_| anyhow!("Failed to parse Content-Type"))?)
        .body(String::from(body))?;

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = AsyncSmtpTransport::relay(&smtp_server)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    log::info!("Email sent successfully to {} with subject: '{}'", to_email, subject);
    Ok(())
}

/// Broadcasts a WebSocket message to all connected clients.
/// Messages are now of type `WebSocketServerResponse`.
async fn broadcast_ws_message<T: Serialize>(clients: Clients, response_type: WebSocketServerResponse) {
    let message = serde_json::to_string(&response_type).expect("Failed to serialize WS message");
    let message_ws = AxumWsMessage::Text(message);

    // Acquire read lock to iterate over clients
    let clients_map = clients.read().await;
    for (&_id, tx) in clients_map.iter() {
        if let Err(e) = tx.send(message_ws.clone()).await {
            log::warn!("Failed to send message to client: {}", e);
        }
    }
}

/// The main Kafka consumer loop that processes events.
async fn kafka_consumer_task(clients: Clients) {
    let kafka_bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
        .map_err(|_| anyhow!("KAFKA_BOOTSTRAP_SERVERS not set"))
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            "localhost:9092".to_string()
        });
    let kafka_topic = std::env::var("KAFKA_TOPIC")
        .map_err(|_| anyhow!("KAFKA_TOPIC not set"))
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            "notification_events".to_string()
        });

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "notification_group_rust")
        .set("bootstrap.servers", &kafka_bootstrap_servers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "latest")
        .set("session.timeout.ms", "6000") // Example: longer session timeout
        .set("enable.auto.commit", "true")
        .set("allow.auto.create.topics", "true") // Allow Kafka to create topic if it doesn't exist
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&kafka_topic])
        .expect("Can't subscribe to specified topic");

    log::info!("Kafka consumer connected to topic: {}", kafka_topic);
    broadcast_ws_message(
        clients.clone(),
        WebSocketServerResponse::KafkaStatusUpdate(KafkaStatusUpdate {
            status: "Running".to_string(),
            message: "Consumer loop started".to_string(),
            error: None,
        }),
    )
    .await;

    // Stream messages from Kafka
    loop {
        match consumer.recv().await {
            Ok(message) => {
                let current_timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let payload = match message.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        log::error!("Error decoding payload: {:?}", e);
                        broadcast_ws_message(
                            clients.clone(),
                            WebSocketServerResponse::NotificationFailure(NotificationFailure {
                                message: "Error decoding Kafka payload".to_string(),
                                recipient: None,
                                template_id: None,
                                timestamp: current_timestamp,
                                error: format!("{:?}", e),
                            }),
                        )
                        .await;
                        continue;
                    }
                    None => {
                        log::warn!("Received empty payload.");
                        continue;
                    }
                };

                log::info!("Processing Kafka event: {}", payload);

                let event: KafkaEvent = match serde_json::from_str(payload) {
                    Ok(e) => e,
                    Err(e) => {
                        log::error!("Failed to parse Kafka event JSON: {} - {}", payload, e);
                        broadcast_ws_message(
                            clients.clone(),
                            WebSocketServerResponse::NotificationFailure(NotificationFailure {
                                message: "Failed to parse Kafka event JSON".to_string(),
                                recipient: None,
                                template_id: None,
                                timestamp: current_timestamp,
                                error: e.to_string(),
                            }),
                        )
                        .await;
                        continue;
                    }
                };

                if event.email.is_empty() || event.template_id.is_empty() {
                    log::warn!(
                        "Skipping event due to missing 'email' or 'template_id': {:?}",
                        event
                    );
                    continue;
                }

                // 1. Fetch email template
                let template = match fetch_email_template(&event.template_id).await {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!(
                            "Error fetching template for {}: {}",
                            event.template_id,
                            e
                        );
                        broadcast_ws_message(
                            clients.clone(),
                            WebSocketServerResponse::NotificationFailure(NotificationFailure {
                                message: format!(
                                    "Failed to fetch template for {} (ID: {})",
                                    event.email, event.template_id
                                ),
                                recipient: Some(event.email.clone()),
                                template_id: Some(event.template_id.clone()),
                                timestamp: current_timestamp,
                                error: e.to_string(),
                            }),
                        )
                        .await;
                        continue;
                    }
                };

                // Simple template rendering
                let mut rendered_subject = template.subject;
                let mut rendered_body = template.body;

                for (key, value) in &event.data {
                    let placeholder = format!("{{{{{}}}}}", key);
                    rendered_subject = rendered_subject.replace(&placeholder, value);
                    rendered_body = rendered_body.replace(&placeholder, value);
                }

                // 2. Send email
                let email_sent = match send_email(&event.email, &rendered_subject, &rendered_body).await {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send email to {}: {}", event.email, e);
                        broadcast_ws_message(
                            clients.clone(),
                            WebSocketServerResponse::NotificationFailure(NotificationFailure {
                                message: format!("Failed to send email to {}", event.email),
                                recipient: Some(event.email.clone()),
                                template_id: Some(event.template_id.clone()),
                                timestamp: current_timestamp,
                                error: e.to_string(),
                            }),
                        )
                        .await;
                        false
                    }
                };

                // 3. Notify clients via WebSocket if successful
                if email_sent {
                    broadcast_ws_message(
                        clients.clone(),
                        WebSocketServerResponse::NotificationSuccess(NotificationSuccess {
                            message: format!(
                                "Email sent to {} for template {}",
                                event.email, event.template_id
                            ),
                            recipient: event.email,
                            template_id: event.template_id,
                            timestamp: current_timestamp,
                        }),
                    )
                    .await;
                }
            }
            Err(e) => {
                log::error!("Kafka error: {}", e);
                // Optionally, emit a Kafka status error to clients
                tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Prevent busy-looping on persistent errors
            }
        }
    }
}

/// Placeholder for real token validation logic.
/// In a real app, this would verify a JWT, query a database, or call an auth service.
/// Returns Ok(user_id_string) if valid, Err(message) otherwise.
async fn validate_access_token(token: &str) -> Result<String, String> {
    if token == "your_super_secret_token" {
        // In a real app, you might decode the token to get a user ID
        Ok("authenticated_user_123".to_string())
    } else {
        Err("Invalid access token".to_string())
    }
}

/// Handler for WebSocket connections.
/// This handler now simply upgrades the connection. Authentication happens via payload.
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(clients): State<Clients>,
) -> Response {
    log::info!("Received WebSocket upgrade request for /ws route. Upgrading connection.");
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, clients)).into_response()
}

/// Handles a new WebSocket connection.
/// Now performs authentication based on a message received over the WebSocket.
async fn handle_websocket_connection(mut ws: WebSocket, clients: Clients) {
    let client_id = NEXT_CLIENT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut authenticated_user_id: Option<String> = None;
    let mut is_authenticated = false;

    log::info!("Client {} connected to WebSocket. Awaiting authentication message.", client_id);

    // Use an MPSC channel to send messages from other tasks to this client's WebSocket
    let (tx, mut rx) = mpsc::unbounded_channel();
    clients.write().await.insert(client_id, tx.clone()); // Insert sender regardless of auth state

    // Task to send messages from the MPSC channel to the WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = ws.send(message).await {
                log::warn!("Failed to send message to client {}: {}", client_id, e);
                break;
            }
        }
        log::info!("Client {} send task finished.", client_id);
    });

    // Task to receive messages from the WebSocket (e.g., pings, close, or auth messages)
    let recv_task = tokio::spawn(async move {
        let current_client_id = client_id; // Capture client_id for logging within this task
        let mut current_authenticated_user_id = authenticated_user_id.clone();
        let mut current_is_authenticated = is_authenticated;
        let local_tx_for_responses = tx; // Use the local sender for client-specific responses

        while let Some(result) = ws.recv().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    log::warn!("WebSocket receive error for client {}: {}", current_client_id, e);
                    break;
                }
            };

            if msg.is_close() {
                log::info!("Client {} sent close frame.", current_client_id);
                break; // Client requested to close
            }

            if let Some(text_msg) = msg.to_str().ok() {
                if !current_is_authenticated {
                    // Try to authenticate if not already authenticated
                    match serde_json::from_str::<WebSocketClientAction>(text_msg) {
                        Ok(WebSocketClientAction::Authenticate { token }) => {
                            match validate_access_token(&token).await {
                                Ok(user_id) => {
                                    current_authenticated_user_id = Some(user_id.clone());
                                    current_is_authenticated = true;
                                    log::info!("Client {} authenticated as user: {}", current_client_id, user_id);
                                    let auth_success_msg = WebSocketServerResponse::AuthSuccess { user_id };
                                    if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&auth_success_msg).unwrap())).await {
                                        log::error!("Failed to send auth_success to client {}: {}", current_client_id, e);
                                    }
                                },
                                Err(e) => {
                                    log::warn!("Client {} authentication failed: {}", current_client_id, e);
                                    let auth_failure_msg = WebSocketServerResponse::AuthFailure { error: e };
                                    if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&auth_failure_msg).unwrap())).await {
                                        log::error!("Failed to send auth_failure to client {}: {}", current_client_id, e);
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            // Message was not an authentication action or invalid format
                            log::warn!("Client {} (unauthenticated) sent unhandled message: {}", current_client_id, text_msg);
                            // Optionally send an error response here for unauthenticated clients sending non-auth messages
                        }
                    }
                } else {
                    // Client is authenticated, process other messages if any
                    log::info!("Client {} (User: {:?}) sent message: {}", current_client_id, current_authenticated_user_id, text_msg);
                    // Add logic for other messages from authenticated clients here
                }
            } else {
                log::info!("Client {} sent non-text message.", current_client_id);
            }
        }
        log::info!("Client {} receive task finished, disconnecting.", current_client_id);
    });

    // Wait for either send or receive task to complete (meaning connection closed)
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Remove client from shared state on disconnect
    clients.write().await.remove(&client_id);
    log::info!("Client {} (User: {:?}) disconnected.", client_id, authenticated_user_id);
}

// HTML dashboard route
async fn index_handler() -> Html<String> {
    let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Notification Server Status (Rust - Axum)</title>
            <style>
                body {
                    font-family: 'Inter', sans-serif;
                    margin: 0;
                    padding: 20px;
                    background-color: #e2e8f0; /* Tailwind gray-200 */
                    color: #2d3748; /* Tailwind gray-800 */
                    display: flex;
                    justify-content: center;
                    align-items: flex-start;
                    min-height: 100vh;
                }
                .container {
                    background: #ffffff;
                    padding: 30px;
                    border-radius: 12px;
                    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
                    max-width: 900px;
                    width: 100%;
                    margin-top: 20px;
                }
                h1 {
                    color: #2c5282; /* Tailwind blue-800 */
                    font-size: 2.25rem; /* Tailwind text-4xl */
                    margin-bottom: 20px;
                    text-align: center;
                }
                p {
                    font-size: 1.125rem; /* Tailwind text-lg */
                    line-height: 1.6;
                    text-align: center;
                    margin-bottom: 20px;
                }
                .status-section {
                    margin-top: 20px;
                    padding: 15px;
                    border-radius: 8px;
                    background-color: #f7fafc; /* Tailwind gray-100 */
                    border: 1px solid #cbd5e0; /* Tailwind gray-300 */
                    text-align: center;
                }
                .status-label {
                    font-weight: bold;
                    color: #4a5568; /* Tailwind gray-700 */
                }
                .log-area {
                    margin-top: 20px;
                    background: #f0f4f8; /* Tailwind gray-100 */
                    padding: 15px;
                    border-radius: 8px;
                    border: 1px solid #cbd5e0; /* Tailwind gray-300 */
                    max-height: 500px;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column-reverse; /* New items appear at the top */
                }
                .log-entry {
                    margin-bottom: 10px;
                    padding: 8px 12px;
                    border-radius: 6px;
                    font-size: 0.9rem;
                    word-break: break-word;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
                }
                .log-success {
                    background-color: #d4edda; /* Tailwind green-100 */
                    color: #155724; /* Dark green text */
                    border: 1px solid #c3e6cb;
                }
                .log-info {
                    background-color: #d1ecf1; /* Tailwind blue-100 */
                    color: #0c5460; /* Dark blue text */
                    border: 1px solid #bee5eb;
                }
                .log-error, .log-failure {
                    background-color: #f8d7da; /* Tailwind red-100 */
                    color: #721c24; /* Dark red text */
                    border: 1px solid #f5c6cb;
                }
                @media (max-width: 768px) {
                    .container {
                        padding: 20px;
                        margin-top: 10px;
                    }
                    h1 {
                        font-size: 1.75rem; /* Tailwind text-3xl */
                    }
                    p {
                        font-size: 1rem; /* Tailwind text-base */
                    }
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Notification Server Live Dashboard</h1>
                <p>This dashboard provides real-time updates on the server's Kafka consumer status and live email sending events.</p>

                <div class="status-section">
                    <span class="status-label">Kafka Consumer Status: </span>
                    <span id="kafka-status">Connecting...</span>
                </div>

                <hr style="margin: 30px 0; border-top: 1px solid #cbd5e0;">

                <h2>Live Notifications:</h2>
                <div id="notifications" class="log-area">
                    <div class="log-entry log-info">Waiting for events...</div>
                </div>
            </div>

            <script type="text/javascript">
                // This is the client-side JavaScript for the dashboard.
                // It now attempts to send an "authenticate" message over the WebSocket
                // after the connection is established.

                // IMPORTANT: In a real application, you would obtain this token
                // after a user successfully logs in (e.g., from an API response).
                const ACCESS_TOKEN = "your_super_secret_token"; // Use a valid token for successful connection
                // const ACCESS_TOKEN = "invalid_token"; // Uncomment to test authentication failure

                const socket = new WebSocket('ws://' + window.location.host + '/ws');
                const notificationsDiv = document.getElementById('notifications');
                const kafkaStatusSpan = document.getElementById('kafka-status');
                let isAuthenticated = false; // Track client-side authentication status

                // Helper to add log entries
                function addLogEntry(message, type) {
                    const entry = document.createElement('div');
                    entry.className = 'log-entry ' + (type || 'log-info');
                    entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
                    notificationsDiv.prepend(entry); // Add to top
                    // Keep only a reasonable number of entries (e.g., 50)
                    if (notificationsDiv.children.length > 50) {
                        notificationsDiv.removeChild(notificationsDiv.lastChild);
                    }
                }

                socket.onopen = function(event) {
                    console.log('Connected to WebSocket server');
                    addLogEntry('Connected to server. Sending authentication message...', 'log-info');
                    // Send authentication action immediately after connection opens
                    const authMessage = {
                        action: "authenticate",
                        token: ACCESS_TOKEN
                    };
                    socket.send(JSON.stringify(authMessage));
                };

                socket.onmessage = function(event) {
                    try {
                        const data = JSON.parse(event.data);
                        console.log('Received WebSocket message:', data);

                        switch (data.type) {
                            case 'authSuccess':
                                isAuthenticated = true;
                                const user_id = data.user_id;
                                addLogEntry(`Authentication successful for user: ${user_id}`, 'log-success');
                                break;
                            case 'authFailure':
                                isAuthenticated = false;
                                const error = data.error;
                                addLogEntry(`Authentication failed: ${error}`, 'log-error');
                                break;
                            case 'notificationSuccess':
                                const successPayload = data.payload;
                                addLogEntry(`SUCCESS: Email to ${successPayload.recipient} (Template: ${successPayload.template_id}). Message: ${successPayload.message}`, 'log-success');
                                break;
                            case 'notificationFailure':
                                const failurePayload = data.payload;
                                addLogEntry(`FAILURE: ${failurePayload.message} (Recipient: ${failurePayload.recipient || 'N/A'}, Template: ${failurePayload.template_id || 'N/A'}). Error: ${failurePayload.error || 'Unknown'}`, 'log-error');
                                break;
                            case 'kafkaStatusUpdate':
                                const kafkaPayload = data.payload;
                                kafkaStatusSpan.textContent = kafkaPayload.status;
                                if (kafkaPayload.status === 'Running') {
                                    kafkaStatusSpan.className = 'log-success';
                                    addLogEntry(`Kafka Consumer Status: ${kafkaPayload.status}. ${kafkaPayload.message || ''}`, 'log-info');
                                } else {
                                    kafkaStatusSpan.className = 'log-error';
                                    addLogEntry(`Kafka Consumer Status: ${kafkaPayload.status}. ${kafkaPayload.message || ''}. Error: ${kafkaPayload.error || 'N/A'}`, 'log-error');
                                }
                                break;
                            default:
                                addLogEntry(`Received unhandled message type: ${data.type}`, 'log-info');
                        }
                    } catch (e) {
                        console.error("Error parsing WebSocket message:", e, event.data);
                        addLogEntry(`Error processing server message: ${event.data}`, 'log-error');
                    }
                };

                socket.onclose = function(event) {
                    console.log('Disconnected from WebSocket server:', event.code, event.reason);
                    if (event.code === 1000) { // Normal closure
                        addLogEntry('Disconnected from server.', 'log-info');
                    } else if (event.code === 1006) { // Abnormally closed (e.g., server rejected)
                        addLogEntry('Connection closed abnormally. Check server logs or token.', 'log-error');
                    } else if (event.code === 1008 || event.code === 1011) { // Policy Violation or Internal Error
                        addLogEntry(`WebSocket error: ${event.reason || 'Server rejected connection.'} (Code: ${event.code})`, 'log-error');
                    } else {
                         addLogEntry('Disconnected from server. Attempting to reconnect...', 'log-error');
                    }
                    isAuthenticated = false; // Reset auth status on close
                    // Simple reconnect logic (for production, use exponential backoff)
                    setTimeout(() => {
                        window.location.reload(); // Simplest way to reconnect for this demo
                    }, 5000);
                };

                socket.onerror = function(error) {
                    console.error('WebSocket error:', error);
                    addLogEntry('WebSocket encountered an error.', 'log-error');
                };
            </script>
        </body>
        </html>
    "#.to_string();
    Html(html_content)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Rust Notification Server with Axum...");

    // Shared state for WebSocket clients
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    // Spawn the Kafka consumer task
    let clients_for_kafka = clients.clone();
    tokio::spawn(async move {
        kafka_consumer_task(clients_for_kafka).await;
    });

    // Build the Axum router
    let app = Router::new()
        .route("/", get(index_handler)) // Route for the HTML dashboard
        .route("/ws", get(ws_handler)) // Route for WebSocket connections
        .with_state(clients) // Share the clients state with handlers
        .layer(CorsLayer::permissive()); // Allow CORS for all origins (for development)

    // Start the server
    let addr = ([0, 0, 0, 0], 5000); // Listen on 0.0.0.0:5000
    let listener = tokio::net::TcpListener::bind(addr).await?;
    log::info!("Server listening on http://{}:{}", addr.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("."), addr.1);
    axum::serve(listener, app).await?;

    Ok(())
}
