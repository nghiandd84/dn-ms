use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum PaymentCoreEventMessage {
    Succeeded { message: PaymentSucceededMessage },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentSucceededMessage {
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub currency: String,
}

pub const PRODUCER_KEY: &str = "payment_core";
