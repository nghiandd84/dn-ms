use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_merchant_entities::api_key::{ApiKeyForCreateDto, ApiKeyForUpdateDto, ModelOptionDto};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct ApiKeyData {
    pub id: Option<i32>,
    pub environment: Option<String>,
    pub api_key: Option<String>,
    pub merchant_id: Option<String>,
    pub created_at: Option<DateTime>,
}

impl Into<ApiKeyData> for ModelOptionDto {
    fn into(self) -> ApiKeyData {
        ApiKeyData {
            id: self.id,
            environment: self.environment,
            api_key: self.api_key,
            merchant_id: self.merchant_id,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ApiKeyForCreateRequest {
    #[validate(length(min = 1, message = "environment is required"))]
    pub environment: String,
    #[validate(length(min = 1, message = "api_key is required"))]
    pub api_key: String,
    #[validate(length(min = 1, message = "merchant_id is required"))]
    pub merchant_id: String,
}

impl Into<ApiKeyForCreateDto> for ApiKeyForCreateRequest {
    fn into(self) -> ApiKeyForCreateDto {
        ApiKeyForCreateDto {
            environment: self.environment,
            api_key: self.api_key,
            merchant_id: self.merchant_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ApiKeyForUpdateRequest {
    #[validate(length(min = 1, message = "environment is required"))]
    pub environment: Option<String>,
    #[validate(length(min = 1, message = "api_key is required"))]
    pub api_key: Option<String>,
}

impl Into<ApiKeyForUpdateDto> for ApiKeyForUpdateRequest {
    fn into(self) -> ApiKeyForUpdateDto {
        ApiKeyForUpdateDto {
            environment: self.environment,
            api_key: self.api_key,
        }
    }
}
