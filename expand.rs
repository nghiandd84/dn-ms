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
use serde_json::json;
use std::time::Duration;

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
    data: HashMap<String, String>,
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
    timestamp: u64,
}

#[derive(Debug, Serialize)]
struct NotificationFailure {
    message: String,
    recipient: Option<String>,
    template_id: Option<String>,
    timestamp: u64,
    error: String,
}

#[derive(Debug, Serialize)]
struct KafkaStatusUpdate {
    status: String,
    message: String,
    error: Option<String>,
}

// --- New Structs for WebSocket Authentication and Ping/Pong Messages ---

#[derive(Debug, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
enum WebSocketClientAction {
    Authenticate { token: String },
    Ping,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum WebSocketServerResponse {
    AuthSuccess { user_id: String },
    AuthFailure { error: String },
    NotificationSuccess(NotificationSuccess),
    NotificationFailure(NotificationFailure),
    KafkaStatusUpdate(KafkaStatusUpdate),
    Pong,
}

// --- NEW: Shared WebSocket State now maps user IDs to a list of senders ---
type Clients = Arc<RwLock<HashMap<String, Vec<mpsc::UnboundedSender<AxumWsMessage>>>>>;

/// Fetches an email template from an external API.
async fn fetch_email_template(template_id: &str) -> Result<EmailTemplate> {
    let api_url = std::env::var("EMAIL_TEMPLATE_API_URL")
        .map_err(|_| anyhow!("EMAIL_TEMPLATE_API_URL not set"))?;
    let url = format!("{}/{}", api_url, template_id);
    log::info!("Fetching template from: {}", url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
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

/// NEW: Sends a WebSocket message to all connections for a specific user ID.
async fn send_ws_message_to_user(clients: Clients, user_id: &str, response_type: WebSocketServerResponse) {
    let message = serde_json::to_string(&response_type).expect("Failed to serialize WS message");
    let message_ws = AxumWsMessage::Text(message);

    // Acquire read lock to access the map
    let clients_map = clients.read().await;

    // Find the list of senders for the given user ID
    if let Some(senders) = clients_map.get(user_id) {
        for tx in senders.iter() {
            if let Err(e) = tx.send(message_ws.clone()).await {
                log::warn!("Failed to send message to client for user {}: {}", user_id, e);
            }
        }
    } else {
        log::warn!("Attempted to send message to non-existent user: {}", user_id);
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
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("allow.auto.create.topics", "true")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&kafka_topic])
        .expect("Can't subscribe to specified topic");

    log::info!("Kafka consumer connected to topic: {}", kafka_topic);
    // TODO: This message is broadcast to all users regardless of auth. This is OK for status.
    broadcast_all_ws_message(
        clients.clone(),
        WebSocketServerResponse::KafkaStatusUpdate(KafkaStatusUpdate {
            status: "Running".to_string(),
            message: "Consumer loop started".to_string(),
            error: None,
        }),
    )
    .await;

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
                        // TODO: Update to send to a specific user if the user info is in the payload.
                        broadcast_all_ws_message(
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
                        // TODO: Update to send to a specific user if the user info is in the payload.
                        broadcast_all_ws_message(
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

                let template = match fetch_email_template(&event.template_id).await {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!(
                            "Error fetching template for {}: {}",
                            event.template_id,
                            e
                        );
                        // TODO: Update to send to a specific user if the user info is in the payload.
                        broadcast_all_ws_message(
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

                let mut rendered_subject = template.subject;
                let mut rendered_body = template.body;

                for (key, value) in &event.data {
                    let placeholder = format!("{{{{{}}}}}", key);
                    rendered_subject = rendered_subject.replace(&placeholder, value);
                    rendered_body = rendered_body.replace(&placeholder, value);
                }

                let email_sent = match send_email(&event.email, &rendered_subject, &rendered_body).await {
                    Ok(_) => true,
                    Err(e) => {
                        log::error!("Failed to send email to {}: {}", event.email, e);
                        // TODO: Update to send to a specific user if the user info is in the payload.
                        broadcast_all_ws_message(
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

                // NEW: Logic to find the user ID and send message only to them
                if email_sent {
                    // NOTE: This assumes `event.email` can be used to uniquely identify a user.
                    // In a real system, you'd likely have a dedicated user_id field in your Kafka event payload.
                    // For now, we'll just use the email as a placeholder for the user ID.
                    send_ws_message_to_user(
                        clients.clone(),
                        &event.email,
                        WebSocketServerResponse::NotificationSuccess(NotificationSuccess {
                            message: format!(
                                "Email sent to {} for template {}",
                                event.email, event.template_id
                            ),
                            recipient: event.email.clone(),
                            template_id: event.template_id,
                            timestamp: current_timestamp,
                        }),
                    )
                    .await;
                }
            }
            Err(e) => {
                log::error!("Kafka error: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

// A new function to broadcast a message to *all* clients.
// Useful for server-wide status messages.
async fn broadcast_all_ws_message(clients: Clients, response_type: WebSocketServerResponse) {
    let message = serde_json::to_string(&response_type).expect("Failed to serialize WS message");
    let message_ws = AxumWsMessage::Text(message);

    let clients_map = clients.read().await;
    for (user_id, senders) in clients_map.iter() {
        for tx in senders.iter() {
            if let Err(e) = tx.send(message_ws.clone()).await {
                log::warn!("Failed to send broadcast message to user {}: {}", user_id, e);
            }
        }
    }
}

/// Placeholder for real token validation logic.
async fn validate_access_token(token: &str) -> Result<String, String> {
    if token == "your_super_secret_token" {
        // In a real app, you would decode the token to get a user ID
        Ok("authenticated_user_123".to_string())
    } else {
        Err("Invalid access token".to_string())
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(clients): State<Clients>,
) -> Response {
    log::info!("Received WebSocket upgrade request for /ws route. Upgrading connection.");
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, clients)).into_response()
}

async fn handle_websocket_connection(mut ws: WebSocket, clients: Clients) {
    let mut authenticated_user_id: Option<String> = None;
    let mut is_authenticated = false;

    let inactivity_timeout_secs: u64 = std::env::var("WS_INACTIVITY_TIMEOUT_SECS")
        .unwrap_or_else(|_| "55".to_string())
        .parse()
        .unwrap_or(55);
    let inactivity_timeout = Duration::from_secs(inactivity_timeout_secs);

    let server_ping_interval = Duration::from_secs(30);

    log::info!("Awaiting authentication message. Inactivity timeout: {:?}", inactivity_timeout);

    let (tx, mut rx) = mpsc::unbounded_channel();
    
    let (mut ws_sender, mut ws_receiver) = ws.split();

    // Task to send messages from the MPSC channel to the WebSocket
    let send_task_user_id = authenticated_user_id.clone();
    let send_task = tokio::spawn(async move {
        let mut last_send_time = tokio::time::Instant::now();
        loop {
            tokio::select! {
                Some(message) = rx.recv() => {
                    if let Err(e) = ws_sender.send(message).await {
                        log::warn!("User {:?} send error: {}", send_task_user_id, e);
                        break;
                    }
                    last_send_time = tokio::time::Instant::now();
                }
                _ = tokio::time::sleep_until(last_send_time + server_ping_interval) => {
                    if let Err(e) = ws_sender.send(AxumWsMessage::Text(serde_json::to_string(&WebSocketServerResponse::Pong).unwrap())).await {
                        log::warn!("User {:?} server-ping send error: {}", send_task_user_id, e);
                        break;
                    }
                    last_send_time = tokio::time::Instant::now();
                }
                else => {
                    break;
                }
            }
        }
    });

    // Task to receive messages from the WebSocket (e.g., pings, close, or auth messages)
    let recv_task_clients = clients.clone();
    let recv_task_user_id = authenticated_user_id.clone();
    let recv_task = tokio::spawn(async move {
        let local_tx_for_responses = tx;
        let mut local_user_id: Option<String> = None;

        loop {
            tokio::select! {
                result = tokio::time::timeout(inactivity_timeout, ws_receiver.next()) => {
                    match result {
                        Ok(Some(Ok(msg))) => {
                            if msg.is_close() {
                                log::info!("User {:?} sent close frame.", local_user_id);
                                break;
                            }
                            if let Some(text_msg) = msg.to_str().ok() {
                                if !is_authenticated {
                                    match serde_json::from_str::<WebSocketClientAction>(text_msg) {
                                        Ok(WebSocketClientAction::Authenticate { token }) => {
                                            match validate_access_token(&token).await {
                                                Ok(user_id) => {
                                                    local_user_id = Some(user_id.clone());
                                                    is_authenticated = true;
                                                    
                                                    // NEW: Add the sender to the user's vector of senders
                                                    let mut clients_map = recv_task_clients.write().await;
                                                    clients_map.entry(user_id.clone()).or_insert_with(Vec::new).push(local_tx_for_responses.clone());

                                                    log::info!("Client authenticated as user: {}", user_id);
                                                    let auth_success_msg = WebSocketServerResponse::AuthSuccess { user_id };
                                                    if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&auth_success_msg).unwrap())).await {
                                                        log::error!("Failed to send auth_success to user {}: {}", local_user_id.as_ref().unwrap(), e);
                                                    }
                                                },
                                                Err(e) => {
                                                    log::warn!("Authentication failed: {}", e);
                                                    let auth_failure_msg = WebSocketServerResponse::AuthFailure { error: e };
                                                    if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&auth_failure_msg).unwrap())).await {
                                                        log::error!("Failed to send auth_failure: {}", e);
                                                    }
                                                }
                                            }
                                        },
                                        Ok(WebSocketClientAction::Ping) => {
                                            log::debug!("Unauthenticated client sent ping.");
                                            let pong_msg = WebSocketServerResponse::Pong;
                                            if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&pong_msg).unwrap())).await {
                                                log::error!("Failed to send pong: {}", e);
                                            }
                                        }
                                        Err(_) => {
                                            log::warn!("Unauthenticated client sent unhandled message: {}", text_msg);
                                        }
                                    }
                                } else {
                                    match serde_json::from_str::<WebSocketClientAction>(text_msg) {
                                        Ok(WebSocketClientAction::Ping) => {
                                            log::debug!("User {:?} sent ping.", local_user_id);
                                            let pong_msg = WebSocketServerResponse::Pong;
                                            if let Err(e) = local_tx_for_responses.send(AxumWsMessage::Text(serde_json::to_string(&pong_msg).unwrap())).await {
                                                log::error!("Failed to send pong to user {:?}: {}", local_user_id, e);
                                            }
                                        }
                                        _ => {
                                            log::info!("User {:?} sent message: {}", local_user_id, text_msg);
                                        }
                                    }
                                }
                            } else {
                                log::info!("User {:?} sent non-text message.", local_user_id);
                            }
                        },
                        Ok(Some(Err(e))) => {
                            log::warn!("User {:?} WebSocket receive error: {}", local_user_id, e);
                            break;
                        },
                        Ok(None) => {
                            log::info!("User {:?} WebSocket stream ended.", local_user_id);
                            break;
                        },
                        Err(_) => {
                            log::info!("User {:?} inactive for {:?}. Disconnecting.", local_user_id, inactivity_timeout);
                            break;
                        }
                    }
                }
                else => {
                    break;
                }
            }
        }
        
        // NEW: Cleanup logic to remove the sender from the user's vector
        if let Some(user_id) = local_user_id {
            let mut clients_map = recv_task_clients.write().await;
            if let Some(senders) = clients_map.get_mut(&user_id) {
                senders.retain(|tx_in_map| !tx_in_map.is_closed());
                if senders.is_empty() {
                    clients_map.remove(&user_id);
                }
            }
            log::info!("User {} disconnected. Remaining connections: {}", user_id, clients_map.get(&user_id).map_or(0, |v| v.len()));
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

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
                    background-color: #e2e8f0;
                    color: #2d3748;
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
                    color: #2c5282;
                    font-size: 2.25rem;
                    margin-bottom: 20px;
                    text-align: center;
                }
                p {
                    font-size: 1.125rem;
                    line-height: 1.6;
                    text-align: center;
                    margin-bottom: 20px;
                }
                .status-section {
                    margin-top: 20px;
                    padding: 15px;
                    border-radius: 8px;
                    background-color: #f7fafc;
                    border: 1px solid #cbd5e0;
                    text-align: center;
                }
                .status-label {
                    font-weight: bold;
                    color: #4a5568;
                }
                .log-area {
                    margin-top: 20px;
                    background: #f0f4f8;
                    padding: 15px;
                    border-radius: 8px;
                    border: 1px solid #cbd5e0;
                    max-height: 500px;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column-reverse;
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
                    background-color: #d4edda;
                    color: #155724;
                    border: 1px solid #c3e6cb;
                }
                .log-info {
                    background-color: #d1ecf1;
                    color: #0c5460;
                    border: 1px solid #bee5eb;
                }
                .log-error, .log-failure {
                    background-color: #f8d7da;
                    color: #721c24;
                    border: 1px solid #f5c6cb;
                }
                @media (max-width: 768px) {
                    .container {
                        padding: 20px;
                        margin-top: 10px;
                    }
                    h1 {
                        font-size: 1.75rem;
                    }
                    p {
                        font-size: 1rem;
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
                const ACCESS_TOKEN = "your_super_secret_token";
                const PING_INTERVAL_MS = 30 * 1000;
                let pingTimer;
                const socket = new WebSocket('ws://' + window.location.host + '/ws');
                const notificationsDiv = document.getElementById('notifications');
                const kafkaStatusSpan = document.getElementById('kafka-status');
                let isAuthenticated = false;

                function addLogEntry(message, type) {
                    const entry = document.createElement('div');
                    entry.className = 'log-entry ' + (type || 'log-info');
                    entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
                    notificationsDiv.prepend(entry);
                    if (notificationsDiv.children.length > 50) {
                        notificationsDiv.removeChild(notificationsDiv.lastChild);
                    }
                }

                socket.onopen = function(event) {
                    console.log('Connected to WebSocket server');
                    addLogEntry('Connected to server. Sending authentication message...', 'log-info');
                    const authMessage = {
                        action: "authenticate",
                        token: ACCESS_TOKEN
                    };
                    socket.send(JSON.stringify(authMessage));

                    if (pingTimer) clearInterval(pingTimer);
                    pingTimer = setInterval(() => {
                        if (socket.readyState === WebSocket.OPEN) {
                            const pingMessage = { action: "ping" };
                            socket.send(JSON.stringify(pingMessage));
                            console.log("Client sent ping.");
                        }
                    }, PING_INTERVAL_MS);
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
                            case 'pong':
                                console.log("Received pong from server.");
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
                    clearInterval(pingTimer);
                    if (event.code === 1000) {
                        addLogEntry('Disconnected from server.', 'log-info');
                    } else if (event.code === 1006) {
                        addLogEntry('Connection closed abnormally. Check server logs or token.', 'log-error');
                    } else if (event.code === 1008 || event.code === 1011) {
                        addLogEntry(`WebSocket error: ${event.reason || 'Server rejected connection.'} (Code: ${event.code})`, 'log-error');
                    } else {
                         addLogEntry('Disconnected from server. Attempting to reconnect...', 'log-error');
                    }
                    isAuthenticated = false;
                    setTimeout(() => {
                        window.location.reload();
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
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Rust Notification Server with Axum...");

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let clients_for_kafka = clients.clone();
    tokio::spawn(async move {
        kafka_consumer_task(clients_for_kafka).await;
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .with_state(clients)
        .layer(CorsLayer::permissive());

    let addr = ([0, 0, 0, 0], 5000);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    log::info!("Server listening on http://{}:{}", addr.0.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("."), addr.1);
    axum::serve(listener, app).await?;

    Ok(())
}
