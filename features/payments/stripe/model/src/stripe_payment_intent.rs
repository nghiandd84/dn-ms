use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_stripe_entities::stripe_payment_intent::{
    ModelOptionDto, StripePaymentIntentForCreateDto, StripePaymentIntentForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct StripePaymentIntentData {
    pub id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub stripe_payment_intent_id: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub client_secret: Option<String>,
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for StripePaymentIntentData {
    fn from(dto: ModelOptionDto) -> Self {
        StripePaymentIntentData {
            id: dto.id,
            payment_id: dto.payment_id,
            stripe_payment_intent_id: dto.stripe_payment_intent_id,
            amount: dto.amount,
            currency: dto.currency,
            status: dto.status,
            client_secret: dto.client_secret,
            metadata: dto.metadata,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripePaymentIntentForCreateRequest {
    pub payment_id: Uuid,
    #[validate(length(
        min = 1,
        max = 255,
        code = "stripe_payment_intent_id_length",
        message = "stripe_payment_intent_id must be between 1 and 255 characters"
    ))]
    pub stripe_payment_intent_id: String,
    #[validate(range(
        min = 1,
        code = "stripe_payment_intent_amount_positive",
        message = "amount must be greater than 0"
    ))]
    pub amount: i64,
    #[validate(length(
        min = 3,
        max = 3,
        code = "stripe_payment_intent_currency_length",
        message = "currency must be a 3-letter ISO code"
    ))]
    pub currency: String,
    #[validate(length(
        min = 1,
        max = 50,
        code = "stripe_payment_intent_status_length",
        message = "status must be between 1 and 50 characters"
    ))]
    pub status: String,
    #[validate(length(
        min = 1,
        max = 2000,
        code = "stripe_payment_intent_client_secret_length",
        message = "client_secret must be between 1 and 2000 characters"
    ))]
    pub client_secret: String,
    pub metadata: Option<Json>,
}

impl From<StripePaymentIntentForCreateRequest> for StripePaymentIntentForCreateDto {
    fn from(req: StripePaymentIntentForCreateRequest) -> Self {
        StripePaymentIntentForCreateDto {
            payment_id: req.payment_id,
            stripe_payment_intent_id: req.stripe_payment_intent_id,
            amount: req.amount,
            currency: req.currency,
            status: req.status,
            client_secret: req.client_secret,
            metadata: req.metadata.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct StripePaymentIntentForUpdateRequest {
    pub status: Option<String>,
    pub metadata: Option<Json>,
}

impl From<StripePaymentIntentForUpdateRequest> for StripePaymentIntentForUpdateDto {
    fn from(req: StripePaymentIntentForUpdateRequest) -> Self {
        StripePaymentIntentForUpdateDto {
            status: req.status,
            metadata: req.metadata,
        }
    }
}
