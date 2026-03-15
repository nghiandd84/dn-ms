use std::collections::HashMap;

use shared_shared_app::state::AppState;

use features_payments_stripe_model::state::{PaymentsStripeAppState, PaymentsStripeCacheState};
use features_payments_stripe_stream::StripeEventMessage;

pub async fn handle_event_consumer_message(
    _message: StripeEventMessage,
    _app_state: AppState<PaymentsStripeAppState, PaymentsStripeCacheState>,
    _headers: Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Implement event consumer logic for Stripe
    Ok(())
}