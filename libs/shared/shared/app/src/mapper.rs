use std::sync::Arc;

use axum::{
    body::to_bytes,
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, to_value, Value};
use tracing::debug;

use shared_shared_data_error::app::AppError;

pub async fn main_response_mapper(uri: Uri, _req_method: Method, res: Response) -> Response {
    let path = uri.path().replace("/", "_").trim_matches('_').to_string();
    debug!(
        "main_response_mapper: path: {}, method: {}",
        path, _req_method
    );
    let headers = res.headers().clone();
    debug!("Response headers: {:#?}", headers);

    let app_error = res.extensions().get::<Arc<AppError>>().map(Arc::as_ref);
    let client_status_error = app_error.map(|e| e.status_and_error());
    match client_status_error {
        Some((status_code, client_error)) => {
            debug!(
                "Mapping AppError to client response: status_code: {}, client_error: {:?}",
                status_code, client_error
            );
            let client_error = to_value(client_error).ok();
            let error_type = client_error
                .as_ref()
                .and_then(|v| v.get("error_type"))
                .cloned();
            let details = client_error.as_ref().and_then(|v| v.get("details"));

            let error_body = json!({
              "data" : {
                "details" : details,
                "error_type" : error_type,
              },
              "status" : 0
            });
            let mut response = (status_code, Json(error_body)).into_response();
            *response.headers_mut() = headers.clone();
            response
        }
        None => {
            if uri.path().starts_with("/swagger-ui/")
                || uri.path().starts_with("/api-docs")
                || uri.path() == "/ws"
            {
                debug!("Skipping response mapping for static files or WebSocket endpoint.");
                return res;
            }

            let status = res.status();

            let body = to_bytes(res.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            let body_string = String::from_utf8(body.to_vec()).unwrap_or_default();
            let data: Value = serde_json::from_str(&body_string).unwrap_or(Value::Null);

            let json_response = json!({
              "status" : 1,
              "data" : data
            });
            // Return headers as well
            let mut response = (status, Json(json_response)).into_response();
            *response.headers_mut() = headers.clone();
            response
        }
    }
}
