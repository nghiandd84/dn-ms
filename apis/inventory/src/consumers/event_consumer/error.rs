use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventError {
    NotFoundClient { user_id: Uuid },
    FailedToCreateSeats
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::NotFoundClient { user_id } => {
                write!(f, "Client not found for user_id: {}", user_id)
            },
            EventError::FailedToCreateSeats => {
                write!(f, "Failed to create seats for the event")
            }
        }
    }
}

impl std::error::Error for EventError {}
