use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_wallet_entities::top_up_transaction::{
    ModelOptionDto, TopUpTransactionForCreateDto, TopUpTransactionForUpdateDto,
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct TopUpTransactionForCreateRequest {
    pub amount: f32,
    #[validate(length(min = 1))]
    pub method: String, // CARD, UPI, BANK_TRANSFER, CASH
    #[serde(default)]
    pub payment_provider_id: Option<String>,
    #[serde(default)]
    pub payment_transaction_id: Option<String>,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct TopUpTransactionForUpdateRequest {
    pub status: Option<String>,
    pub completed_at: Option<DateTime>,
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TopUpTransactionData {
    pub id: Option<Uuid>,
    pub wallet_id: Option<Uuid>,
    pub amount: Option<f32>,
    pub method: Option<String>,
    pub payment_provider_id: Option<String>,
    pub payment_transaction_id: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,
}

impl Into<TopUpTransactionData> for ModelOptionDto {
    fn into(self) -> TopUpTransactionData {
        TopUpTransactionData {
            id: self.id,
            wallet_id: self.wallet_id,
            amount: self.amount,
            method: self.method,
            payment_provider_id: self.payment_provider_id,
            payment_transaction_id: self.payment_transaction_id,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            completed_at: self.completed_at,
            ..Default::default()
        }
    }
}

impl Into<TopUpTransactionForCreateDto> for TopUpTransactionForCreateRequest {
    fn into(self) -> TopUpTransactionForCreateDto {
        TopUpTransactionForCreateDto {
            wallet_id: Uuid::nil(), // Will be set by the service
            amount: self.amount,
            method: self.method,
            payment_provider_id: self.payment_provider_id.unwrap_or_default(),
            payment_transaction_id: self.payment_transaction_id.unwrap_or_default(),
            status: "PENDING".to_string(),
        }
    }
}

impl Into<TopUpTransactionForUpdateDto> for TopUpTransactionForUpdateRequest {
    fn into(self) -> TopUpTransactionForUpdateDto {
        TopUpTransactionForUpdateDto {
            status: self.status,
            completed_at: self.completed_at,
        }
    }
}
