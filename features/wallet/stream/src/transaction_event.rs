use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCreatedEvent {
    pub transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: String,
    pub amount: String,
    pub currency: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionUpdatedEvent {
    pub transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub status: Option<String>,
    pub description: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSucceededEvent {
    pub transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: String,
    pub currency: String,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFailedEvent {
    pub transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub reason: String,
    pub failed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum TransactionEvent {
    #[serde(rename = "transaction.created")]
    Created(TransactionCreatedEvent),
    #[serde(rename = "transaction.updated")]
    Updated(TransactionUpdatedEvent),
    #[serde(rename = "transaction.succeeded")]
    Succeeded(TransactionSucceededEvent),
    #[serde(rename = "transaction.failed")]
    Failed(TransactionFailedEvent),
}
