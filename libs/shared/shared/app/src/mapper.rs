use std::sync::Arc;

use crate::state::AppState;
use axum::{
    body::to_bytes,
    extract::{Request, State},
    http::{Method, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, to_value, Value};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;
use shared_shared_data_app::{ctx::Ctx, error::AppError};

pub async fn main_response_mapper(uri: Uri, _req_method: Method, res: Response) -> Response {
    let uuid = Uuid::new_v4();
    let app_error = res.extensions().get::<Arc<AppError>>().map(Arc::as_ref);
    let client_status_error = app_error.map(|e| e.status_and_error());
    match client_status_error {
        Some((status_code, client_error)) => {
            debug!("Error: {:?} - {:?} - {:?}", uuid, status_code, app_error);
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let details = client_error.as_ref().and_then(|v| v.get("details"));

            let error_body = json!({
              "req_id" : uuid.to_string(),
              "data" : {
                "details" : details,
                "message" : message,
              },
              "status" : 0
            });
            // log -> uuid, http_path, http_method, res, error
            // let _ = log_request(uuid, uri, req_method, error_body.clone(), 0).await;
            (status_code, Json(error_body)).into_response()
        }
        None => {
            debug!(
                "Success: {:?} - {:?} - path {:?}",
                uuid,
                res.status(),
                uri.path()
            );
            if uri.path().starts_with("/swagger-ui/") || uri.path().starts_with("/api-docs") {
                return res;
            }
            let status = res.status();
            let body = to_bytes(res.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            let body_string = String::from_utf8(body.to_vec()).unwrap_or_default();
            let data: Value = serde_json::from_str(&body_string).unwrap_or(Value::Null);
            let json_response = json!({
              "req_id" : uuid.to_string(),
              "status" : 1,
              "data" : data,
              "metadata" : null // pagination
            });
            // let _ = log_request(uuid, uri, req_method, json_response.clone(), 1).await;
            (status, Json(json_response)).into_response()
        }
    }
}

pub async fn mw_ctx_resolver(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    debug!("mw_ctx_resolver");
    // Create ctx from token
    let ctx = Ctx::new(1);
    let _result_ctx: Result<Ctx> = Ok(ctx);
    // let result_ctx: Result<Ctx, Error> = Err(Error::CtxNotInRequestExt);
    // request.extensions_mut().insert(result_ctx);
    let response = next.run(request).await;

    response
}
