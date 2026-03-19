use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum MerchantEventMessage {
    New { message: NewMerchantEventMessage },
    Update { message: ChangeMerchantEventMessage },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewMerchantEventMessage {
    pub id: Uuid,
    pub business_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeMerchantEventMessage {
    pub id: Uuid,
    pub business_name: String,
}

pub const PRODUCER_KEY: &str = "merchant";
