use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_payments_core_entities::payment_attempt::{
    ModelOptionDto, PaymentAttemptForCreateDto, PaymentAttemptForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PaymentAttemptData {
    pub id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub provider: Option<String>,
    pub raw_request: Option<Json>,
    pub raw_response: Option<Json>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime>,
}
impl Into<PaymentAttemptData> for ModelOptionDto {
    fn into(self) -> PaymentAttemptData {
        PaymentAttemptData {
            id: self.id,
            payment_id: self.payment_id,
            provider: self.provider,
            raw_request: self.raw_request,
            raw_response: self.raw_response,
            success: self.success,
            error_message: self.error_message,
            created_at: self.created_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentAttemptForCreateRequest {
    pub payment_id: Uuid,
    pub provider: String,
    pub raw_request: Json,
    pub raw_response: Json,
    pub success: bool,
    pub error_message: String,
}

impl Into<PaymentAttemptForCreateDto> for PaymentAttemptForCreateRequest {
    fn into(self) -> PaymentAttemptForCreateDto {
        PaymentAttemptForCreateDto {
            payment_id: self.payment_id,
            provider: self.provider,
            raw_request: self.raw_request,
            raw_response: self.raw_response,
            success: self.success,
            error_message: self.error_message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentAttemptForUpdateRequest {
    pub payment_id: Option<Uuid>,
    pub provider: Option<String>,
    pub raw_request: Option<Json>,
    pub raw_response: Option<Json>,
    pub success: Option<bool>,
    pub error_message: Option<String>,
}

impl Into<PaymentAttemptForUpdateDto> for PaymentAttemptForUpdateRequest {
    fn into(self) -> PaymentAttemptForUpdateDto {
        PaymentAttemptForUpdateDto {
            payment_id: self.payment_id,
            provider: self.provider,
            raw_request: self.raw_request,
            raw_response: self.raw_response,
            success: self.success,
            error_message: self.error_message,
        }
    }
}
