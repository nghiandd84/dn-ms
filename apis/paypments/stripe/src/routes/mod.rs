use axum::Router;
use std::sync::Arc;

use crate::app::AppState;

pub mod checkout;
pub mod refund;
pub mod payments;
pub mod webhook;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(checkout::checkout_routes())
        .merge(refund::refund_routes())
        .merge(payments::payments_routes())
        .merge(webhook::webhook_routes())
}