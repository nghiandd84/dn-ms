use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpTransactionInitiatedEvent {
    pub top_up_transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: String,
    pub method: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpTransactionSucceededEvent {
    pub top_up_transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: String,
    pub method: String,
    pub payment_provider_id: Option<String>,
    pub payment_transaction_id: Option<String>,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpTransactionFailedEvent {
    pub top_up_transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: String,
    pub reason: String,
    pub failed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopUpTransactionUpdatedEvent {
    pub top_up_transaction_id: Uuid,
    pub wallet_id: Uuid,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum TopUpTransactionEvent {
    #[serde(rename = "top_up_transaction.initiated")]
    Initiated(TopUpTransactionInitiatedEvent),
    #[serde(rename = "top_up_transaction.succeeded")]
    Succeeded(TopUpTransactionSucceededEvent),
    #[serde(rename = "top_up_transaction.failed")]
    Failed(TopUpTransactionFailedEvent),
    #[serde(rename = "top_up_transaction.updated")]
    Updated(TopUpTransactionUpdatedEvent),
}
