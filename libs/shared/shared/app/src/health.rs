use axum::response::Json;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::instrument;
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

#[instrument]
fn get_message() -> String {
    "Service is healthy".to_string()
}

#[derive(Serialize, ToSchema, Response)]
struct Health {
    message: String,
}
