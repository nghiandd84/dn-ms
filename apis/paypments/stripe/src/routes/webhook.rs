use axum::{extract::State, http::HeaderMap, Router};
use shared_shared_auth::permission::PublicAccess;
use std::sync::Arc;
use stripe::{EventType, Webhook};

use crate::app::AppState;

pub async fn handle_webhook(
    _public: PublicAccess,
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    payload: String,
) -> Result<&'static str, String> {
    let sig = headers
        .get("stripe-signature")
        .ok_or("Missing signature")?
        .to_str()
        .map_err(|_| "Invalid signature")?;
    let endpoint_secret =
        std::env::var("STRIPE_WEBHOOK_SECRET").map_err(|_| "Missing webhook secret")?;

    let event =
        Webhook::construct_event(&payload, sig, &endpoint_secret).map_err(|e| e.to_string())?;

    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            // Handle payment success
            println!("Payment succeeded: {:?}", event.data);
        }
        EventType::PaymentIntentPaymentFailed => {
            // Handle payment failure
            println!("Payment failed: {:?}", event.data);
        }
        _ => {
            // Other events
        }
    }

    Ok("ok")
}

pub fn webhook_routes() -> Router<Arc<AppState>> {
    Router::new().route("/webhook", axum::routing::post(handle_webhook))
}
