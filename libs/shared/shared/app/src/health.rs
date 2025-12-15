use axum::response::Json;
use axum_tracing_opentelemetry::tracing_opentelemetry_instrumentation_sdk;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::{event, info_span, instrument, Level};
use utoipa::ToSchema;

use shared_shared_macro::Response;

#[utoipa::path(
    get,
    path = "/healthchecker",
    tag = "Health Checker",
    responses(
        (status = 200, description= "Health Checker", body= HealthResponse),       
    )
)]
#[instrument(level = Level::INFO)]
pub async fn health_checker_handler() -> Json<Value> {
    // Get the current trace id
    let trace_id = tracing_opentelemetry_instrumentation_sdk::find_current_trace_id();
    event!(Level::INFO, trace_id = ?trace_id, "Current Trace ID logged in health checker");

    let message = get_message();
    let health = Health {
        message: message.clone(),
    };
    Json(json!(health))
}

#[instrument(level = Level::INFO)]
fn get_message() -> String {
    let span = info_span!("message_span");
    let _guard = span.enter();

    event!(Level::INFO, "something happened inside my_span");
    "Service is healthy".to_string()
}

#[derive(Serialize, ToSchema, Response)]
struct Health {
    message: String,
}
