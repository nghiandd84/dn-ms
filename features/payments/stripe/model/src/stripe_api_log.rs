use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_stripe_entities::stripe_api_log::{
    ModelOptionDto, StripeApiLogForCreateDto, StripeApiLogForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct StripeApiLogData {
    pub id: Option<Uuid>,
    pub endpoint: Option<String>,
    pub method: Option<String>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub stripe_request_id: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for StripeApiLogData {
    fn from(dto: ModelOptionDto) -> Self {
        StripeApiLogData {
            id: dto.id,
            endpoint: dto.endpoint,
            method: dto.method,
            request_body: dto.request_body,
            response_body: dto.response_body,
            status_code: dto.status_code,
            error_message: dto.error_message,
            stripe_request_id: dto.stripe_request_id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripeApiLogForCreateRequest {
    #[validate(length(
        min = 1,
        max = 500,
        code = "stripe_api_log_endpoint_length",
        message = "endpoint must be between 1 and 500 characters"
    ))]
    pub endpoint: String,
    #[validate(length(
        min = 1,
        max = 20,
        code = "stripe_api_log_method_length",
        message = "method must be between 1 and 20 characters"
    ))]
    pub method: String,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    #[validate(range(
        min = 100,
        max = 599,
        code = "stripe_api_log_status_code_range",
        message = "status_code must be a valid HTTP status code"
    ))]
    pub status_code: i32,
    pub error_message: Option<String>,
    #[validate(length(
        min = 1,
        max = 255,
        code = "stripe_api_log_request_id_length",
        message = "stripe_request_id must be between 1 and 255 characters"
    ))]
    pub stripe_request_id: String,
}

impl From<StripeApiLogForCreateRequest> for StripeApiLogForCreateDto {
    fn from(req: StripeApiLogForCreateRequest) -> Self {
        StripeApiLogForCreateDto {
            endpoint: req.endpoint,
            method: req.method,
            request_body: req.request_body.unwrap_or_default(),
            response_body: req.response_body.unwrap_or_default(),
            status_code: req.status_code,
            error_message: req.error_message.unwrap_or_default(),
            stripe_request_id: req.stripe_request_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripeApiLogForUpdateRequest {
    pub response_body: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
}

impl From<StripeApiLogForUpdateRequest> for StripeApiLogForUpdateDto {
    fn from(req: StripeApiLogForUpdateRequest) -> Self {
        StripeApiLogForUpdateDto {
            response_body: req.response_body,
            status_code: req.status_code,
            error_message: req.error_message,
        }
    }
}
