use axum::{extract::State, Json, Router};
use serde_json;
use shared_shared_auth::permission::PublicAccess;
use std::sync::Arc;
use stripe::{ListPaymentIntents, PaymentIntent};

use crate::app::AppState;

pub async fn list_payments(
    _public: PublicAccess,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, String> {
    let client = &state.stripe_client;

    let payments = PaymentIntent::list(client, &ListPaymentIntents { ..Default::default() }).await.map_err(|e| e.to_string())?;

    let list = payments.data.into_iter().map(|p| serde_json::to_value(p).unwrap()).collect();

    Ok(Json(list))
}

pub fn payments_routes() -> Router<Arc<AppState>> {
    Router::new().route("/payments", axum::routing::get(list_payments))
}