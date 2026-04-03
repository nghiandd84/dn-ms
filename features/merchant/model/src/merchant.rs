use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_merchant_entities::merchant::{
    MerchantForCreateDto, MerchantForUpdateDto, ModelOptionDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct MerchantData {
    pub id: Option<String>,
    pub business_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub business_type: Option<String>,
    pub kyc_status: Option<String>,
    pub kyc_verified_at: Option<DateTime>,
    pub status: Option<String>,
    pub created_at: Option<DateTime>,
}

impl Into<MerchantData> for ModelOptionDto {
    fn into(self) -> MerchantData {
        MerchantData {
            id: self.id,
            business_name: self.business_name,
            email: self.email,
            phone: self.phone,
            business_type: self.business_type,
            kyc_status: self.kyc_status,
            kyc_verified_at: self.kyc_verified_at,
            status: self.status,
            created_at: self.created_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct MerchantForCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "merchant_business_name_length",
        message = "business_name must be between 1 and 255 characters"
    ))]
    pub business_name: String,
    #[validate(email(
        code = "merchant_email_invalid",
        message = "email must be a valid email address"
    ))]
    pub email: String,
    #[validate(length(
        min = 1,
        max = 20,
        code = "merchant_phone_length",
        message = "phone must be between 1 and 20 characters"
    ))]
    pub phone: String,
    #[validate(length(
        min = 1,
        max = 100,
        code = "merchant_business_type_length",
        message = "business_type must be between 1 and 100 characters"
    ))]
    pub business_type: String,
    #[validate(length(
        min = 1,
        max = 50,
        code = "merchant_kyc_status_length",
        message = "kyc_status must be between 1 and 50 characters"
    ))]
    pub kyc_status: String,
    #[validate(length(
        min = 1,
        max = 50,
        code = "merchant_status_length",
        message = "status must be between 1 and 50 characters"
    ))]
    pub status: String,
}

impl Into<MerchantForCreateDto> for MerchantForCreateRequest {
    fn into(self) -> MerchantForCreateDto {
        MerchantForCreateDto {
            business_name: self.business_name,
            email: self.email,
            phone: self.phone,
            business_type: self.business_type,
            kyc_status: self.kyc_status,
            status: self.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct MerchantForUpdateRequest {
    pub business_name: Option<String>,
    #[validate(email(message = "email must be a valid email address"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub business_type: Option<String>,
    pub kyc_status: Option<String>,
    pub kyc_verified_at: Option<DateTime>,
    pub status: Option<String>,
}

impl Into<MerchantForUpdateDto> for MerchantForUpdateRequest {
    fn into(self) -> MerchantForUpdateDto {
        MerchantForUpdateDto {
            business_name: self.business_name,
            email: self.email,
            phone: self.phone,
            business_type: self.business_type,
            kyc_status: self.kyc_status,
            kyc_verified_at: self.kyc_verified_at,
            status: self.status,
        }
    }
}
