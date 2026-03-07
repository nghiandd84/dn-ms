use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_payments_core_entities::payment_method::{
    ModelOptionDto, PaymentMethodForCreateDto, PaymentMethodForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PaymentMethodData {
    pub id: Option<Uuid>,
    pub display_name: Option<String>,
    pub provider_name: Option<String>,
    pub provider_config: Option<Json>,
    pub supported_countries: Option<VecString>,
    pub supported_currencies: Option<VecString>,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
    pub fee_percentage: Option<f32>,
    pub icon_url: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
impl Into<PaymentMethodData> for ModelOptionDto {
    fn into(self) -> PaymentMethodData {
        PaymentMethodData {
            id: self.id,
            display_name: self.display_name,
            provider_name: self.provider_name,
            provider_config: self.provider_config,
            supported_countries: self.supported_countries,
            supported_currencies: self.supported_currencies,
            priority: self.priority,
            is_active: self.is_active,
            fee_percentage: self.fee_percentage,
            icon_url: self.icon_url,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentMethodForCreateRequest {
    pub display_name: String,
    pub provider_name: String,
    pub provider_config: Json,
    pub supported_countries: Vec<String>,
    pub supported_currencies: Vec<String>,
    pub priority: i32,
    pub is_active: bool,
    pub fee_percentage: f32,
    pub icon_url: String,
}

impl Into<PaymentMethodForCreateDto> for PaymentMethodForCreateRequest {
    fn into(self) -> PaymentMethodForCreateDto {
        PaymentMethodForCreateDto {
            display_name: self.display_name,
            provider_name: self.provider_name,
            provider_config: self.provider_config,
            supported_countries: self.supported_countries,
            supported_currencies: self.supported_currencies,
            priority: self.priority,
            is_active: self.is_active,
            fee_percentage: self.fee_percentage,
            icon_url: self.icon_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaymentMethodForUpdateRequest {
    pub display_name: Option<String>,
    pub provider_name: Option<String>,
    pub provider_config: Option<Json>,
    pub supported_countries: Option<Vec<String>>,
    pub supported_currencies: Option<Vec<String>>,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
    pub fee_percentage: Option<f32>,
    pub icon_url: Option<String>,
}

impl Into<PaymentMethodForUpdateDto> for PaymentMethodForUpdateRequest {
    fn into(self) -> PaymentMethodForUpdateDto {
        PaymentMethodForUpdateDto {
            display_name: self.display_name,
            provider_name: self.provider_name,
            provider_config: self.provider_config,
            supported_countries: self.supported_countries,
            supported_currencies: self.supported_currencies,
            priority: self.priority,
            is_active: self.is_active,
            fee_percentage: self.fee_percentage,
            icon_url: self.icon_url,
        }
    }
}