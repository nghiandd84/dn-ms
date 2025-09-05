use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "event_type", rename_all = "camelCase")]
pub enum KafkaEvent {
    NotificationEvent { user_id: Option<Uuid>, message: String },
    DepositSuccess { user_id: Uuid, platform: String },
    WithdrawalSuccess { user_id: Uuid, platform: String },
}
