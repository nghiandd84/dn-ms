use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_macro::Response;

use features_payments_paypal_entities::paypal_refund::{
    ModelOptionDto, PaypalRefundForCreateDto, PaypalRefundForUpdateDto,
};

#[derive(Serialize, Debug, ToSchema, Default, Response)]
pub struct PaypalRefundData {
    pub id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub paypal_refund_id: Option<String>,
    pub paypal_capture_id: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl From<ModelOptionDto> for PaypalRefundData {
    fn from(dto: ModelOptionDto) -> Self {
        PaypalRefundData {
            id: dto.id,
            payment_id: dto.payment_id,
            paypal_refund_id: dto.paypal_refund_id,
            paypal_capture_id: dto.paypal_capture_id,
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
pub struct PaypalRefundForCreateRequest {
    pub payment_id: Uuid,
    pub paypal_refund_id: String,
    pub paypal_capture_id: String,
    #[validate(range(min = 1, message = "amount must be greater than 0"))]
    pub amount: i64,
    #[validate(length(min = 3, max = 3, message = "currency must be a 3-letter ISO code"))]
    pub currency: String,
    pub status: String,
    pub reason: Option<String>,
    pub metadata: Option<Json>,
}

impl From<PaypalRefundForCreateRequest> for PaypalRefundForCreateDto {
    fn from(req: PaypalRefundForCreateRequest) -> Self {
        PaypalRefundForCreateDto {
            payment_id: req.payment_id,
            paypal_refund_id: req.paypal_refund_id,
            paypal_capture_id: req.paypal_capture_id,
            amount: req.amount,
            currency: req.currency,
            status: req.status,
            reason: req.reason.unwrap_or_default(),
            metadata: req.metadata.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PaypalRefundForUpdateRequest {
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl From<PaypalRefundForUpdateRequest> for PaypalRefundForUpdateDto {
    fn from(req: PaypalRefundForUpdateRequest) -> Self {
        PaypalRefundForUpdateDto {
            status: req.status,
            metadata: req.metadata,
        }
    }
}
