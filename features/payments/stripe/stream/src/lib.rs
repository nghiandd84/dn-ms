use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum StripeEventMessage {
    PaymentIntent { message: PaymentIntentEventMessage },
    Refund { message: RefundEventMessage },
    Webhook { message: WebhookEventMessage },
    ApiLog { message: ApiLogEventMessage },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentIntentEventMessage {
    pub id: Uuid,
    pub stripe_payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundEventMessage {
    pub id: Uuid,
    pub stripe_refund_id: String,
    pub payment_intent_id: String,
    pub amount: i64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookEventMessage {
    pub id: Uuid,
    pub stripe_event_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiLogEventMessage {
    pub id: Uuid,
    pub endpoint: String,
    pub method: String,
    pub status_code: u16,
    pub response_time: i64,
}

pub const PRODUCER_KEY: &str = "stripe";
