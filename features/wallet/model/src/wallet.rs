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

use features_wallet_entities::wallet::{ModelOptionDto, WalletForCreateDto, WalletForUpdateDto};

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct WalletForCreateRequest {
    #[validate(length(min = 1))]
    pub currency: String,
    #[serde(default)]
    pub balance: Option<String>,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct WalletForUpdateRequest {
    pub balance: Option<String>,
    pub currency: Option<String>,
    pub is_active: Option<bool>,
}

impl Into<WalletForUpdateDto> for WalletForUpdateRequest {
    fn into(self) -> WalletForUpdateDto {
        WalletForUpdateDto {
            balance: self.balance,
            currency: self.currency,
            is_active: self.is_active,
        }
    }
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct WalletData {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub currency: Option<String>,
    pub balance: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<WalletData> for ModelOptionDto {
    fn into(self) -> WalletData {
        WalletData {
            id: self.id,
            user_id: self.user_id,
            currency: self.currency,
            balance: self.balance,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

impl Into<WalletForCreateDto> for WalletForCreateRequest {
    fn into(self) -> WalletForCreateDto {
        WalletForCreateDto {
            user_id: Uuid::nil(), // Will be set by the service using authenticated user
            currency: self.currency,
            balance: self.balance.unwrap_or_else(|| "0".to_string()).into(),
        }
    }
}
