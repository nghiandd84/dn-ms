use std::sync::Arc;

use axum::{
    body::to_bytes,
    extract::Request,
    http::{Method, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_tracing_opentelemetry::tracing_opentelemetry_instrumentation_sdk;
use serde_json::{json, to_value, Value};
use tracing::{debug, info};

use shared_shared_data_app::ctx::Ctx;
use shared_shared_data_app::result::Result;
use shared_shared_data_error::app::AppError;

pub async fn main_response_mapper(uri: Uri, _req_method: Method, res: Response) -> Response {
    // debug!(
    //     "main_response_mapper: uri: {}, method: {}",
    //     uri, _req_method
    // );
    // let uuid = Uuid::new_v4();
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
            // log -> uuid, http_path, http_method, res, error
            // let _ = log_request(uuid, uri, req_method, error_body.clone(), 0).await;
            (status_code, Json(error_body)).into_response()
        }
        None => {
            if uri.path().starts_with("/swagger-ui/")
                || uri.path().starts_with("/api-docs")
                || uri.path() == "/ws"
            {
                debug!("Skipping response mapping for static files or WebSocket endpoint.");
                return res;
            }

            // Can get current trace id if  RUST_LOG=trace is set
            let trace_id_str = tracing_opentelemetry_instrumentation_sdk::find_current_trace_id()
                .unwrap_or_default();
            info!("Current Trace ID: {}", trace_id_str);

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
            (status, Json(json_response)).into_response()
        }
    }
}

pub async fn mw_ctx_resolver(
    // TODO try to use app state in here
    // State(_state): State<AppState<C>>,
    req: Request,
    next: Next,
) -> Response {
    // Create ctx from token
    let ctx = Ctx::new(1);
    let _result_ctx: Result<Ctx> = Ok(ctx);
    // let result_ctx: Result<Ctx, Error> = Err(Error::CtxNotInRequestExt);
    // request.extensions_mut().insert(result_ctx);
    // debug!("mw_ctx_resolver: ctx inserted into request extensions");
    let res = next.run(req).await;
    // debug!("mw_ctx_resolver: response generated");

    res
}
