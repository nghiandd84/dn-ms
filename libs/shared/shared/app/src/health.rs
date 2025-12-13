use axum::response::Json;
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
pub async fn health_checker_handler() -> Json<Value> {
    let message = get_message();
    let health = Health {
        message: message.clone(),
    };
    Json(json!(health))
}

#[instrument(level = "info")]
fn get_message() -> String {
    let span = info_span!("message_span", user_id = "Unknow",  request_id = %uuid::Uuid::new_v4());
    let _guard = span.enter();
    event!(Level::INFO, "something happened inside my_span");
    "Service is healthy".to_string()
}

#[derive(Serialize, ToSchema, Response)]
struct Health {
    message: String,
}
