use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum EventMessage {
    New { message: NewEventMessage },
    Update { message: ChangeEventMessage },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEventMessage {
    pub id: Uuid,
    pub total_seats: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeEventMessage {
    pub id: Uuid,
    pub total_seats: String,
}

pub const PRODUCER_KEY: &str = "event";
