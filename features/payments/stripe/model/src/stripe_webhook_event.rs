use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_stripe_entities::stripe_webhook_event::{
    ModelOptionDto, StripeWebhookEventForCreateDto, StripeWebhookEventForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct StripeWebhookEventData {
    pub id: Option<Uuid>,
    pub stripe_event_id: Option<String>,
    pub event_type: Option<String>,
    pub event_data: Option<Json>,
    pub processed: Option<bool>,
    pub processing_error: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for StripeWebhookEventData {
    fn from(dto: ModelOptionDto) -> Self {
        StripeWebhookEventData {
            id: dto.id,
            stripe_event_id: dto.stripe_event_id,
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
pub struct StripeWebhookEventForCreateRequest {
    #[validate(length(
        min = 1,
        max = 255,
        code = "stripe_event_id_length",
        message = "stripe_event_id must be between 1 and 255 characters"
    ))]
    pub stripe_event_id: String,
    #[validate(length(
        min = 1,
        max = 100,
        code = "webhook_event_type_length",
        message = "event_type must be between 1 and 100 characters"
    ))]
    pub event_type: String,
    pub event_data: Json,
    pub processed: Option<bool>,
    #[validate(length(
        max = 2000,
        code = "webhook_processing_error_length",
        message = "processing_error must not exceed 2000 characters"
    ))]
    pub processing_error: Option<String>,
}

impl From<StripeWebhookEventForCreateRequest> for StripeWebhookEventForCreateDto {
    fn from(req: StripeWebhookEventForCreateRequest) -> Self {
        StripeWebhookEventForCreateDto {
            stripe_event_id: req.stripe_event_id,
            event_type: req.event_type,
            event_data: req.event_data,
            processed: req.processed.unwrap_or(false),
            processing_error: req.processing_error.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripeWebhookEventForUpdateRequest {
    pub processed: Option<bool>,
    pub processing_error: Option<String>,
}

impl From<StripeWebhookEventForUpdateRequest> for StripeWebhookEventForUpdateDto {
    fn from(req: StripeWebhookEventForUpdateRequest) -> Self {
        StripeWebhookEventForUpdateDto {
            processed: req.processed,
            processing_error: req.processing_error,
        }
    }
}
