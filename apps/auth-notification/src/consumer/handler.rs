use tracing::{debug, error};

use features_auth_model::state::AuthAppState;
use features_auth_stream::AuthMessage;

use crate::consumer::error::ConsumerError;

pub async fn handle_consumer_message(
    message: AuthMessage,
    auth_state: AuthAppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = match message {
        AuthMessage::SignIn { message } => {
            debug!("Handling sign-in message: {:?}", message);
            Ok(())
        }
        AuthMessage::SignUp { message } => {
            debug!("Handling sign-up message");
            // handle_signup_message(notification_state, message).await
            Ok(())
        }
    };
    result
}
/*
async fn handle_payment_message<'a>(
    notification_state: Arc<RwLock<NotificationState>>,
    user_id: uuid::Uuid,
    platform: String,
    message: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_sender = {
        let state_read_guard = notification_state.write().unwrap();
        let client_sender = state_read_guard.get_client_sender_by_user_id(user_id);
        client_sender
    };
    if client_sender.is_none() {
        debug!("No client sender found for user_id {:?}", user_id);
        return Err(Box::new(ConsumerError::NotFoundClient { user_id }));
    }
    let client_sender = client_sender.unwrap();
    let websocket_message = ServerResponse::Payment { platform, message };
    if let Err(e) = client_sender.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        debug!("Failed to send message to user {:?}: {}", user_id, e);
        return Err(Box::new(ConsumerError::FailedToSendMessage {
            user_id,
            message: e.to_string(),
        }));
    }
    Ok(())
}

async fn handle_notification_message<'a>(
    notification_state: Arc<RwLock<NotificationState>>,
    user_id: uuid::Uuid,
    message: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_sender = {
        let state_read_guard = notification_state.write().unwrap();
        let client_sender = state_read_guard.get_client_sender_by_user_id(user_id);
        client_sender
    };
    if client_sender.is_none() {
        debug!("No client sender found for user_id {:?}", user_id);
        return Err(Box::new(ConsumerError::NotFoundClient { user_id }));
    }
    let client_sender = client_sender.unwrap();
    let websocket_message = ServerResponse::Notification { message };
    if let Err(e) = client_sender.send(Message::Text(
        serde_json::to_string(&websocket_message).unwrap().into(),
    )) {
        error!("Failed to send message to user {:?}: {}", user_id, e);
        return Err(Box::new(ConsumerError::FailedToSendMessage {
            user_id,
            message: e.to_string(),
        }));
    }
    Ok(())
}
 */
