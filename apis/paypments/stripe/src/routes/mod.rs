use axum::Router;

use features_payments_stripe_model::state::{PaymentsStripeAppState, PaymentsStripeCacheState};
use shared_shared_app::state::AppState;

pub mod stripe_api_log;
pub mod stripe_payment_intent;
pub mod stripe_refund;
pub mod stripe_webhook_event;

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .merge(stripe_payment_intent::routes(app_state))
        .merge(stripe_refund::routes(app_state))
        .merge(stripe_webhook_event::routes(app_state))
        .merge(stripe_api_log::routes(app_state))
}
