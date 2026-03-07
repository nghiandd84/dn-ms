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

use features_payments_core_entities::payment::{
    ModelOptionDto, PaymentForCreateDto, PaymentForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PaymentData {
    pub id: Option<Uuid>,
    pub transaction_id: Option<String>,
    pub user_id: Option<Uuid>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub provider_name: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
impl Into<PaymentData> for ModelOptionDto {
    fn into(self) -> PaymentData {
        PaymentData {
            id: self.id,
            transaction_id: self.transaction_id,
            user_id: self.user_id,
            amount: self.amount,
            currency: self.currency,
            status: self.status,
            provider_name: self.provider_name,
            gateway_transaction_id: self.gateway_transaction_id,
            idempotency_key: self.idempotency_key,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentForCreateRequest {
    pub transaction_id: String,
    pub user_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub provider_name: String,
    pub gateway_transaction_id: String,
    pub idempotency_key: String,
}

impl Into<PaymentForCreateDto> for PaymentForCreateRequest {
    fn into(self) -> PaymentForCreateDto {
        PaymentForCreateDto {
            transaction_id: self.transaction_id,
            user_id: self.user_id,
            amount: self.amount,
            currency: self.currency,
            status: self.status,
            provider_name: self.provider_name,
            gateway_transaction_id: self.gateway_transaction_id,
            idempotency_key: self.idempotency_key,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentForUpdateRequest {
    pub transaction_id: Option<String>,
    pub user_id: Option<Uuid>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub provider_name: Option<String>,
    pub gateway_transaction_id: Option<String>,
    pub idempotency_key: Option<String>,
}

impl Into<PaymentForUpdateDto> for PaymentForUpdateRequest {
    fn into(self) -> PaymentForUpdateDto {
        PaymentForUpdateDto {
            transaction_id: self.transaction_id,
            user_id: self.user_id,
            amount: self.amount,
            currency: self.currency,
            status: self.status,
            provider_name: self.provider_name,
            gateway_transaction_id: self.gateway_transaction_id,
            idempotency_key: self.idempotency_key,
        }
    }
}