use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreatedEvent {
    pub wallet_id: Uuid,
    pub user_id: Uuid,
    pub currency: String,
    pub balance: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletUpdatedEvent {
    pub wallet_id: Uuid,
    pub currency: Option<String>,
    pub balance: Option<String>,
    pub is_active: Option<bool>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDeletedEvent {
    pub wallet_id: Uuid,
    pub user_id: Uuid,
    pub deleted_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum WalletEvent {
    #[serde(rename = "wallet.created")]
    Created(WalletCreatedEvent),
    #[serde(rename = "wallet.updated")]
    Updated(WalletUpdatedEvent),
    #[serde(rename = "wallet.deleted")]
    Deleted(WalletDeletedEvent),
}
