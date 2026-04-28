use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use stripe::{CreateRefund, ListRefunds, Refund, PaymentIntentId};

use crate::app::AppState;

#[derive(Deserialize)]
pub struct RefundRequest {
    pub payment_intent_id: String,
    pub amount: Option<i64>,
}

#[derive(Serialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub status: String,
}

pub async fn create_refund(
    _public: PublicAccess,
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefundRequest>,
) -> Result<Json<RefundResponse>, String> {
    let client = &state.stripe_client;

    let payment_intent_id = PaymentIntentId::from_str(&req.payment_intent_id)
        .map_err(|_| "Invalid payment intent ID".to_string())?;
    // Check for existing refunds to prevent duplicates
    let refunds = Refund::list(
        client,
        &ListRefunds {
            payment_intent: Some(payment_intent_id.clone()),

            ..Default::default()
        },
    )
    .await
    .map_err(|e| e.to_string())?;
    if !refunds.data.is_empty() {
        return Err("Refund already exists for this payment".to_string());
    }

    let refund = CreateRefund {
        payment_intent: Some(payment_intent_id),
        amount: req.amount,
        ..Default::default()
    };

    let refund = Refund::create(client, refund)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(RefundResponse {
        refund_id: refund.id.to_string(),
        status: refund.status.unwrap().to_string(),
    }))
}

pub fn refund_routes() -> Router<Arc<AppState>> {
    Router::new().route("/refund", axum::routing::post(create_refund))
}
