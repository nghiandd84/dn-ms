use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum FeeEventMessage {
    New { message: NewFeeEventMessage },
    Update { message: ChangeFeeEventMessage },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewFeeEventMessage {
    pub id: Uuid,
    pub business_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeFeeEventMessage {
    pub id: Uuid,
    pub business_name: String,
}

pub const PRODUCER_KEY: &str = "fee";
