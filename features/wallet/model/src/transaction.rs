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
    #[validate(length(
        min = 1,
        max = 50,
        code = "transaction_type_length",
        message = "transaction_type must be between 1 and 50 characters"
    ))]
    pub transaction_type: String,
    #[validate(range(
        min = 0.01,
        code = "transaction_amount_positive",
        message = "amount must be greater than 0"
    ))]
    pub amount: f32,
    #[validate(length(
        min = 1,
        max = 10,
        code = "transaction_currency_length",
        message = "currency must be between 1 and 10 characters"
    ))]
    pub currency: String,
    #[validate(length(
        max = 255,
        code = "reference_id_length",
        message = "reference_id must not exceed 255 characters"
    ))]
    pub reference_id: Option<String>,
    #[validate(length(
        max = 1000,
        code = "transaction_description_length",
        message = "description must not exceed 1000 characters"
    ))]
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
    pub amount: Option<f32>,
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
