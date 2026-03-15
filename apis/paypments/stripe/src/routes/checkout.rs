use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::{
    CheckoutSession, CheckoutSessionMode, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsPriceData, CreateCheckoutSessionLineItemsPriceDataProductData,
    Currency,
};

use crate::app::AppState;

#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub amount: i64, // in cents
    pub currency: String,
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Serialize)]
pub struct CheckoutResponse {
    pub session_id: String,
    pub url: String,
}

pub async fn create_checkout(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CheckoutRequest>,
) -> Result<Json<CheckoutResponse>, String> {
    let client = &state.stripe_client;

    let currency = match req.currency.as_str() {
        "usd" => Currency::USD,
        "eur" => Currency::EUR,
        "gbp" => Currency::GBP,
        _ => return Err("Unsupported currency".to_string()),
    };

    let session = CreateCheckoutSession {
        mode: Some(CheckoutSessionMode::Payment),
        line_items: Some(vec![CreateCheckoutSessionLineItems {
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency,
                // TODO Update product_data later
                // product_data: CreateCheckoutSessionLineItemsPriceDataProductData {
                //     name: Some("Payment".to_string()),
                //     ..Default::default()
                // },
                unit_amount: Some(req.amount),
                ..Default::default()
            }),
            quantity: Some(1),
            ..Default::default()
        }]),
        success_url: Some(req.success_url.as_str()),
        cancel_url: Some(req.cancel_url.as_str()),
        ..Default::default()
    };

    let session = CheckoutSession::create(client, session)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(CheckoutResponse {
        session_id: session.id.to_string(),
        url: session.url.ok_or("No URL")?,
    }))
}

pub fn checkout_routes() -> Router<Arc<AppState>> {
    Router::new().route("/checkout", axum::routing::post(create_checkout))
}
