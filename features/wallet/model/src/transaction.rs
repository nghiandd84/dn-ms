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

use features_wallet_entities::transaction::{
    ModelOptionDto, TransactionForCreateDto, TransactionForUpdateDto,
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct TransactionForCreateRequest {
    pub wallet_id: Uuid,
    pub transaction_type: String, // DEPOSIT, WITHDRAWAL, TRANSFER, PAYMENT
    pub amount: String,
    pub currency: String,
    pub reference_id: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct TransactionForUpdateRequest {
    pub status: Option<String>,
    pub description: Option<String>,
}

impl Into<TransactionForUpdateDto> for TransactionForUpdateRequest {
    fn into(self) -> TransactionForUpdateDto {
        TransactionForUpdateDto {
            status: self.status,
            description: self.description,
        }
    }
}


#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct TransactionData {
    pub id: Option<Uuid>,
    pub wallet_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub reference_id: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<TransactionData> for ModelOptionDto {
    fn into(self) -> TransactionData {
        TransactionData {
            id: self.id,
            wallet_id: self.wallet_id,
            transaction_type: self.transaction_type,
            amount: self.amount,
            currency: self.currency,
            status: self.status,
            reference_id: self.reference_id,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

impl Into<TransactionForCreateDto> for TransactionForCreateRequest {
    fn into(self) -> TransactionForCreateDto {
        TransactionForCreateDto {
            wallet_id: self.wallet_id,
            transaction_type: self.transaction_type,
            amount: self.amount,
            currency: self.currency,
            status: "INITIATED".to_string(), // Default status for new transactions
            description: self.description.unwrap_or_default(),
        }
    }
}
