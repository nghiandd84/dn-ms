use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_fee_entities::fee_configuration::{
    FeeConfigurationForCreateDto, FeeConfigurationForUpdateDto, ModelOptionDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct FeeConfigurationData {
    pub id: Option<Uuid>,
    pub merchant_id: Option<String>,
    pub pricing_model: Option<String>,
    pub percentage_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
    pub min_fee: Option<f64>,
    pub max_fee: Option<f64>,
    pub tier_config: Option<Json>,
    pub effective_from: Option<DateTime>,
    pub effective_to: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<FeeConfigurationData> for ModelOptionDto {
    fn into(self) -> FeeConfigurationData {
        FeeConfigurationData {
            id: self.id,
            merchant_id: self.merchant_id,
            pricing_model: self.pricing_model,
            percentage_rate: self.percentage_rate,
            fixed_amount: self.fixed_amount,
            min_fee: self.min_fee,
            max_fee: self.max_fee,
            tier_config: self.tier_config,
            effective_from: self.effective_from,
            effective_to: self.effective_to,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct FeeConfigurationForCreateRequest {
    #[validate(length(min = 1, message = "merchant_id is required"))]
    pub merchant_id: String,
    #[validate(length(min = 1, message = "pricing_model is required"))]
    pub pricing_model: String,
    pub percentage_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
    pub min_fee: Option<f64>,
    pub max_fee: Option<f64>,
    pub tier_config: Option<Json>,
    pub effective_from: Option<DateTime>,
    pub effective_to: Option<DateTime>,
}

impl Into<FeeConfigurationForCreateDto> for FeeConfigurationForCreateRequest {
    fn into(self) -> FeeConfigurationForCreateDto {
        FeeConfigurationForCreateDto {
            merchant_id: self.merchant_id,
            pricing_model: self.pricing_model,
            percentage_rate: self.percentage_rate.unwrap_or_default(),
            fixed_amount: self.fixed_amount.unwrap_or_default(),
            min_fee: self.min_fee.unwrap_or_default(),
            max_fee: self.max_fee.unwrap_or_default(),
            tier_config: self.tier_config.unwrap_or_default(),
            effective_from: self.effective_from.unwrap_or_default(),
            effective_to: self.effective_to.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct FeeConfigurationForUpdateRequest {
    pub merchant_id: Option<String>,
    pub pricing_model: Option<String>,
    pub percentage_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
    pub min_fee: Option<f64>,
    pub max_fee: Option<f64>,
    pub tier_config: Option<Json>,
    pub effective_from: Option<DateTime>,
    pub effective_to: Option<DateTime>,
}

impl Into<FeeConfigurationForUpdateDto> for FeeConfigurationForUpdateRequest {
    fn into(self) -> FeeConfigurationForUpdateDto {
        FeeConfigurationForUpdateDto {
            merchant_id: self.merchant_id,
            pricing_model: self.pricing_model,
            percentage_rate: self.percentage_rate,
            fixed_amount: self.fixed_amount,
            min_fee: self.min_fee,
            max_fee: self.max_fee,
            tier_config: self.tier_config,
            effective_from: self.effective_from,
            effective_to: self.effective_to,
        }
    }
}
