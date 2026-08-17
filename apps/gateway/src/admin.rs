use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request, State as AxumState},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tracing::{debug, error, info};

use crate::{
    config::dn_config::DnConfig,
    gateway::state::{build_gateway_state, GatewayStateStore},
};

pub struct AdminState {
    pub dp: String,
    pub gateway_stores: Vec<Arc<GatewayStateStore>>,
    pub admin_api_key: Option<String>,
}

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

async fn reload(AxumState(state): AxumState<Arc<AdminState>>) -> impl IntoResponse {
    debug!("Admin reload requested");

    let new_dn_config = DnConfig::load_from_path(&state.dp);

    if new_dn_config.gateways.len() != state.gateway_stores.len() {
        error!(
            "Gateway count mismatch: config has {}, running has {}. Full restart required.",
            new_dn_config.gateways.len(),
            state.gateway_stores.len()
        );
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "error": "gateway count mismatch",
                "message": format!(
                    "config has {} gateways, running has {}. A full restart is required to add/remove gateways.",
                    new_dn_config.gateways.len(),
                    state.gateway_stores.len()
                ),
            })),
        );
    }

    let mut reloaded = 0usize;
    for (i, gateway_config) in new_dn_config.gateways.iter().enumerate() {
        let new_state = build_gateway_state(gateway_config.clone());
        state.gateway_stores[i].update_state(new_state).await;
        reloaded += 1;
        info!("Reloaded gateway config: {}", gateway_config.name);
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "reloaded",
            "gateways": reloaded,
        })),
    )
}

/// Middleware: verify X-Admin-Key header against the configured admin API key.
/// Skips check if no key is configured (open mode).
async fn verify_admin_key(
    AxumState(state): AxumState<Arc<AdminState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // If no admin key is configured, allow all requests (open mode)
    if let Some(expected_key) = &state.admin_api_key {
        let provided_key = request
            .headers()
            .get("X-Admin-Key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if provided_key != expected_key {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "unauthorized",
                    "message": "Missing or invalid X-Admin-Key header",
                })),
            ));
        }
    }

    Ok(next.run(request).await)
}

pub fn admin_router(admin_state: Arc<AdminState>) -> Router {
    Router::new()
        .route("/admin/health", get(health))
        .route("/admin/reload", post(reload))
        .layer(axum::middleware::from_fn_with_state(
            admin_state.clone(),
            verify_admin_key,
        ))
        .with_state(admin_state)
}
