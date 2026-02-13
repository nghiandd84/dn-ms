use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventError {
    NotFoundClient { user_id: Uuid },
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::NotFoundClient { user_id } => {
                write!(f, "Client not found for user_id: {}", user_id)
            }
        }
    }
}

impl std::error::Error for EventError {}
