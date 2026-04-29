use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// POST /paypal/flow/initiate
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct InitiatePaymentRequest {
    pub user_id: Uuid,
    #[validate(range(min = 1, message = "amount must be > 0"))]
    pub amount: i64,
    #[validate(length(equal = 3, message = "currency must be 3-letter ISO code"))]
    pub currency: String,
    pub idempotency_key: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InitiatePaymentResponse {
    pub payment_id: Uuid,
    pub paypal_order_id: String,
    pub approval_url: String,
}

/// POST /paypal/flow/capture
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CapturePaymentRequest {
    pub paypal_order_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CapturePaymentResponse {
    pub payment_id: Uuid,
    pub capture_id: String,
    pub status: String,
}

/// POST /paypal/flow/refund
#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RefundPaymentRequest {
    pub payment_id: Uuid,
    pub amount: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefundPaymentResponse {
    pub refund_id: Uuid,
    pub paypal_refund_id: String,
    pub status: String,
}
