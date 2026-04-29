use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_paypal_entities::paypal_order::{
    ModelOptionDto, PaypalOrderForCreateDto, PaypalOrderForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct PaypalOrderData {
    pub id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub paypal_order_id: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub approval_url: Option<String>,
    pub capture_id: Option<String>,
    pub metadata: Option<Json>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for PaypalOrderData {
    fn from(dto: ModelOptionDto) -> Self {
        PaypalOrderData {
            id: dto.id,
            payment_id: dto.payment_id,
            paypal_order_id: dto.paypal_order_id,
            amount: dto.amount,
            currency: dto.currency,
            status: dto.status,
            approval_url: dto.approval_url,
            capture_id: dto.capture_id,
            metadata: dto.metadata,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalOrderForCreateRequest {
    pub payment_id: Uuid,
    pub paypal_order_id: String,
    #[validate(range(min = 1, message = "amount must be greater than 0"))]
    pub amount: i64,
    #[validate(length(min = 3, max = 3, message = "currency must be a 3-letter ISO code"))]
    pub currency: String,
    pub status: String,
    pub approval_url: Option<String>,
    pub capture_id: Option<String>,
    pub metadata: Option<Json>,
}

impl From<PaypalOrderForCreateRequest> for PaypalOrderForCreateDto {
    fn from(req: PaypalOrderForCreateRequest) -> Self {
        PaypalOrderForCreateDto {
            payment_id: req.payment_id,
            paypal_order_id: req.paypal_order_id,
            amount: req.amount,
            currency: req.currency,
            status: req.status,
            approval_url: req.approval_url.unwrap_or_default(),
            capture_id: req.capture_id.unwrap_or_default(),
            metadata: req.metadata.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalOrderForUpdateRequest {
    pub status: Option<String>,
    pub capture_id: Option<String>,
    pub metadata: Option<Json>,
}

impl From<PaypalOrderForUpdateRequest> for PaypalOrderForUpdateDto {
    fn from(req: PaypalOrderForUpdateRequest) -> Self {
        PaypalOrderForUpdateDto {
            status: req.status,
            capture_id: req.capture_id,
            metadata: req.metadata,
        }
    }
}
