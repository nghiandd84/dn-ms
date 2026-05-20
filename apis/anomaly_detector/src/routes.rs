use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{handlers, AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Fingerprint registration (called by browser)
        .route("/fingerprints", post(handlers::register_fingerprint))
        .route("/fingerprints", get(handlers::list_fingerprints))
        .route("/fingerprints/{fingerprint}", get(handlers::get_fingerprint))
        // Block management (called by admin)
        .route("/blocks", post(handlers::create_block))
        .route("/blocks", get(handlers::list_blocks))
        .route("/blocks/{fingerprint}", get(handlers::get_block))
        .route("/blocks/{fingerprint}", delete(handlers::delete_block))
        .with_state(state)
}
