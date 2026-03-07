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

use features_payments_core_entities::payment_method_limit::{
    ModelOptionDto, PaymentMethodLimitForCreateDto, PaymentMethodLimitForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PaymentMethodLimitData {
    pub id: Option<Uuid>,
    pub payment_method_id: Option<Uuid>,
    pub currency: Option<String>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
impl Into<PaymentMethodLimitData> for ModelOptionDto {
    fn into(self) -> PaymentMethodLimitData {
        PaymentMethodLimitData {
            id: self.id,
            payment_method_id: self.payment_method_id,
            currency: self.currency,
            min_amount: self.min_amount,
            max_amount: self.max_amount,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentMethodLimitForCreateRequest {
    pub payment_method_id: Uuid,
    pub currency: String,
    pub min_amount: i64,
    pub max_amount: i64,
}

impl Into<PaymentMethodLimitForCreateDto> for PaymentMethodLimitForCreateRequest {
    fn into(self) -> PaymentMethodLimitForCreateDto {
        PaymentMethodLimitForCreateDto {
            payment_method_id: self.payment_method_id,
            currency: self.currency,
            min_amount: self.min_amount,
            max_amount: self.max_amount,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentMethodLimitForUpdateRequest {
    pub payment_method_id: Option<Uuid>,
    pub currency: Option<String>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
}

impl Into<PaymentMethodLimitForUpdateDto> for PaymentMethodLimitForUpdateRequest {
    fn into(self) -> PaymentMethodLimitForUpdateDto {
        PaymentMethodLimitForUpdateDto {
            payment_method_id: self.payment_method_id,
            currency: self.currency,
            min_amount: self.min_amount,
            max_amount: self.max_amount,
        }
    }
}