use axum::extract::ws::Message;
use tracing::{debug, error};

use features_email_template_model::{
    state::{NotificationCacheState, NotificationState},
    types::{ClientSender, Clients},
};
use shared_shared_app::state::AppState;
use uuid::Uuid;

use crate::consumer::event::KafkaEvent;

pub async fn handler_event<'a>(
    event: KafkaEvent,
    app_state: &'a AppState<NotificationCacheState, NotificationState>,
) {
    let state = app_state.state.as_ref().unwrap();
    let clients = state.get_clients();
    let len = clients.read().await.len();

    debug!("Number of client connect to app{}", len);
    match event {
        KafkaEvent::NotificationEvent { user_id, message } => {
            // Handle notification event
            debug!(
                "Notificate handler: Notification for user {:?}: {}",
                user_id, message
            );
        }
        KafkaEvent::DepositSuccess { user_id, platform } => {
            // Handle deposit success event
            debug!(
                "Notificate handler: Deposit success for user {:?} on platform {}",
                user_id, platform
            );
        }
        KafkaEvent::WithdrawalSuccess { user_id, platform } => {
            // Handle withdrawal success event
            debug!(
                "Notificate handler: Withdrawal success for user {:?} on platform {}",
                user_id, platform
            );
        }
    }
}

async fn handle_deposit_success_event(clients: &Clients, user_id: uuid::Uuid, platform: String) {
    let message = format!("Deposit successful on platform {}", platform);
    // Send message to all connected clients for simplicity
    let clients_read = clients.read().await;
    debug!(
        "Sending deposit success message to user {:?}: {}",
        user_id, message
    );
    // Implement logic to send message to specific user based on user_id
}

async fn handle_withdrawal_success_event(clients: &Clients, user_id: uuid::Uuid, platform: String) {
    let message = format!("Withdrawal successful on platform {}", platform);
    // Send message to all connected clients for simplicity
    let clients_read = clients.read().await;
    debug!(
        "Sending withdrawal success message to user {:?}: {}",
        user_id, message
    );
    // Implement logic to send message to specific user based on user_id
}

async fn handle_notification_event(
    clients: &Clients,
    user_id: Option<uuid::Uuid>,
    message: String,
) {
    // Send message to all connected clients for simplicity
    let clients_read = clients.read().await;
    if user_id.is_none() {
        debug!("Broadcasting message to all clients");
        for (id, sender) in clients_read.iter() {
            // Here you can implement logic to filter clients based on user_id if needed
            if let Err(e) = sender.send(Message::Text(message.clone().into())) {
                error!("Failed to send message to client {}: {}", id, e);
            }
        }
        return;
    }
    let user_id = user_id.unwrap();
    debug!("Sending message to user {:?}: {}", user_id, message);
    // Implement logic to send message to specific user based on user_id
}

fn get_unbound_sender(clients: &Clients, user_id: Uuid) -> Option<ClientSender> {
    None
}
