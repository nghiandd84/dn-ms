use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ConsumerError {
    NotFoundClient { user_id: Uuid },
    FailedToSendMessage { user_id: Uuid, message: String },
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsumerError::NotFoundClient { user_id } => {
                write!(f, "Client not found for user_id: {}", user_id)
            }
            ConsumerError::FailedToSendMessage { user_id, message } => {
                write!(
                    f,
                    "Failed to send message to user_id {}: {}",
                    user_id, message
                )
            }
        }
    }
}

impl std::error::Error for ConsumerError {}
