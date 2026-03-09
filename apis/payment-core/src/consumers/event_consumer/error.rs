use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventError {
    NotFoundClient { user_id: Uuid },
    FailedToProcessPayment
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::NotFoundClient { user_id } => {
                write!(f, "Client not found for user_id: {}", user_id)
            },
            EventError::FailedToProcessPayment => {
                write!(f, "Failed to process payment")
            }
        }
    }
}

impl std::error::Error for EventError {}
