use axum::Router;

use shared_shared_app::state::AppState;
use features_payments_stripe_model::state::{PaymentsStripeAppState, PaymentsStripeCacheState};

pub mod stripe_payment_intent;
pub mod stripe_refund;
pub mod stripe_webhook_event;
pub mod stripe_api_log;

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .merge(stripe_payment_intent::routes(app_state))
        .merge(stripe_refund::routes(app_state))
        .merge(stripe_webhook_event::routes(app_state))
        .merge(stripe_api_log::routes(app_state))
}