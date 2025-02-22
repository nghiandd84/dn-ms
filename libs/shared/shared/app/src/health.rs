use axum::response::Json;
use serde::Serialize;
use serde_json::{json, Value};
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
    const MESSAGE: &str = "OK";
    let health = Health {
        message: MESSAGE.to_string(),
    };
    Json(json!(health))
}

#[derive(Serialize, ToSchema, Response)]
struct Health {
    message: String,
}
