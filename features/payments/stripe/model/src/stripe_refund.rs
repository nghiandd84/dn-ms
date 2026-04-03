use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_stripe_entities::stripe_refund::{
    ModelOptionDto, StripeRefundForCreateDto, StripeRefundForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct StripeRefundData {
    pub id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub stripe_refund_id: Option<String>,
    pub stripe_payment_intent_id: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for StripeRefundData {
    fn from(dto: ModelOptionDto) -> Self {
        StripeRefundData {
            id: dto.id,
            payment_id: dto.payment_id,
            stripe_refund_id: dto.stripe_refund_id,
            stripe_payment_intent_id: dto.stripe_payment_intent_id,
            amount: dto.amount,
            currency: dto.currency,
            status: dto.status,
            reason: dto.reason,
            metadata: dto.metadata,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripeRefundForCreateRequest {
    pub payment_id: Uuid,
    #[validate(length(
        min = 1,
        max = 255,
        code = "stripe_refund_id_length",
        message = "stripe_refund_id must be between 1 and 255 characters"
    ))]
    pub stripe_refund_id: String,
    #[validate(length(
        min = 1,
        max = 255,
        code = "stripe_payment_intent_id_length",
        message = "stripe_payment_intent_id must be between 1 and 255 characters"
    ))]
    pub stripe_payment_intent_id: String,
    #[validate(range(
        min = 1,
        code = "stripe_refund_amount_positive",
        message = "amount must be greater than 0"
    ))]
    pub amount: i64,
    #[validate(length(
        min = 3,
        max = 3,
        code = "stripe_refund_currency_length",
        message = "currency must be a 3-letter ISO code"
    ))]
    pub currency: String,
    #[validate(length(
        min = 1,
        max = 50,
        code = "stripe_refund_status_length",
        message = "status must be between 1 and 50 characters"
    ))]
    pub status: String,
    #[validate(length(
        max = 500,
        code = "stripe_refund_reason_length",
        message = "reason must not exceed 500 characters"
    ))]
    pub reason: Option<String>,
    pub metadata: Option<Json>,
}

impl From<StripeRefundForCreateRequest> for StripeRefundForCreateDto {
    fn from(req: StripeRefundForCreateRequest) -> Self {
        StripeRefundForCreateDto {
            payment_id: req.payment_id,
            stripe_refund_id: req.stripe_refund_id,
            stripe_payment_intent_id: req.stripe_payment_intent_id,
            amount: req.amount,
            currency: req.currency,
            status: req.status,
            reason: req.reason.unwrap_or_default(),
            metadata: req.metadata.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripeRefundForUpdateRequest {
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl From<StripeRefundForUpdateRequest> for StripeRefundForUpdateDto {
    fn from(req: StripeRefundForUpdateRequest) -> Self {
        StripeRefundForUpdateDto {
            status: req.status,
            metadata: req.metadata,
        }
    }
}
