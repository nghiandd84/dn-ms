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

use features_wallet_entities::withdrawal::{
    ModelOptionDto, WithdrawalForCreateDto, WithdrawalForUpdateDto,
};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct WithdrawalForCreateRequest {
    pub wallet_id: Uuid,
    #[validate(range(
        min = 0.01,
        code = "withdrawal_amount_positive",
        message = "amount must be greater than 0"
    ))]
    pub amount: f32,
    pub payment_device_id: Uuid,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct WithdrawalForUpdateRequest {
    pub status: Option<String>,
    pub completed_at: Option<DateTime>,
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct WithdrawalData {
    pub id: Option<Uuid>,
    pub wallet_id: Option<Uuid>,
    pub amount: Option<f32>,
    pub payment_device_id: Option<Uuid>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,
}

impl Into<WithdrawalData> for ModelOptionDto {
    fn into(self) -> WithdrawalData {
        WithdrawalData {
            id: self.id,
            wallet_id: self.wallet_id,
            amount: self.amount,
            payment_device_id: self.payment_device_id,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            completed_at: self.completed_at,
        }
    }
}

impl Into<WithdrawalForCreateDto> for WithdrawalForCreateRequest {
    fn into(self) -> WithdrawalForCreateDto {
        WithdrawalForCreateDto {
            wallet_id: self.wallet_id,
            amount: self.amount,
            payment_device_id: self.payment_device_id,
            status: "PENDING".to_string(),
            completed_at: DateTime::default(),
        }
    }
}

impl Into<WithdrawalForUpdateDto> for WithdrawalForUpdateRequest {
    fn into(self) -> WithdrawalForUpdateDto {
        WithdrawalForUpdateDto {
            status: self.status,
            completed_at: self.completed_at,
        }
    }
}
