use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_paypal_entities::paypal_webhook_event::{
    ModelOptionDto, PaypalWebhookEventForCreateDto, PaypalWebhookEventForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct PaypalWebhookEventData {
    pub id: Option<Uuid>,
    pub paypal_event_id: Option<String>,
    pub event_type: Option<String>,
    pub event_data: Option<Json>,
    pub processed: Option<bool>,
    pub processing_error: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for PaypalWebhookEventData {
    fn from(dto: ModelOptionDto) -> Self {
        PaypalWebhookEventData {
            id: dto.id,
            paypal_event_id: dto.paypal_event_id,
            event_type: dto.event_type,
            event_data: dto.event_data,
            processed: dto.processed,
            processing_error: dto.processing_error,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalWebhookEventForCreateRequest {
    pub paypal_event_id: String,
    pub event_type: String,
    pub event_data: Json,
    pub processed: Option<bool>,
    pub processing_error: Option<String>,
}

impl From<PaypalWebhookEventForCreateRequest> for PaypalWebhookEventForCreateDto {
    fn from(req: PaypalWebhookEventForCreateRequest) -> Self {
        PaypalWebhookEventForCreateDto {
            paypal_event_id: req.paypal_event_id,
            event_type: req.event_type,
            event_data: req.event_data,
            processed: req.processed.unwrap_or(false),
            processing_error: req.processing_error.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalWebhookEventForUpdateRequest {
    pub processed: Option<bool>,
    pub processing_error: Option<String>,
}

impl From<PaypalWebhookEventForUpdateRequest> for PaypalWebhookEventForUpdateDto {
    fn from(req: PaypalWebhookEventForUpdateRequest) -> Self {
        PaypalWebhookEventForUpdateDto {
            processed: req.processed,
            processing_error: req.processing_error,
        }
    }
}
