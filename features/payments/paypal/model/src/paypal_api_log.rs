use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_paypal_entities::paypal_api_log::{
    ModelOptionDto, PaypalApiLogForCreateDto, PaypalApiLogForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct PaypalApiLogData {
    pub id: Option<Uuid>,
    pub endpoint: Option<String>,
    pub method: Option<String>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub paypal_request_id: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for PaypalApiLogData {
    fn from(dto: ModelOptionDto) -> Self {
        PaypalApiLogData {
            id: dto.id,
            endpoint: dto.endpoint,
            method: dto.method,
            request_body: dto.request_body,
            response_body: dto.response_body,
            status_code: dto.status_code,
            error_message: dto.error_message,
            paypal_request_id: dto.paypal_request_id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalApiLogForCreateRequest {
    pub endpoint: String,
    pub method: String,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub status_code: i32,
    pub error_message: Option<String>,
    pub paypal_request_id: String,
}

impl From<PaypalApiLogForCreateRequest> for PaypalApiLogForCreateDto {
    fn from(req: PaypalApiLogForCreateRequest) -> Self {
        PaypalApiLogForCreateDto {
            endpoint: req.endpoint,
            method: req.method,
            request_body: req.request_body.unwrap_or_default(),
            response_body: req.response_body.unwrap_or_default(),
            status_code: req.status_code,
            error_message: req.error_message.unwrap_or_default(),
            paypal_request_id: req.paypal_request_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalApiLogForUpdateRequest {
    pub response_body: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
}

impl From<PaypalApiLogForUpdateRequest> for PaypalApiLogForUpdateDto {
    fn from(req: PaypalApiLogForUpdateRequest) -> Self {
        PaypalApiLogForUpdateDto {
            response_body: req.response_body,
            status_code: req.status_code,
            error_message: req.error_message,
        }
    }
}
