use std::usize;

use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use serde_json::Value;
use sha2::{Digest, Sha256};
use shared_shared_extractor::IdempotencyKey;
use tracing::debug;

use features_wallet_model::idempotency::IdempotencyKeyForCreateRequest;
use features_wallet_service::IdempotencyService;

fn calculate_request_hash(method: &str, uri: &str, body_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(method.as_bytes());
    hasher.update(b"|");
    hasher.update(uri.as_bytes());
    hasher.update(b"|");
    hasher.update(body_bytes);
    format!("{:x}", hasher.finalize())
}

pub async fn idempotency_tracking_middleware(req: Request, next: Next) -> impl IntoResponse {
    debug!(
        "Idempotency middleware invoked for path: {}",
        req.uri().path()
    );

    let (parts, body) = req.into_parts();
    let body_bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();

    let key = IdempotencyKey::get_idempotency_key(&parts);
    debug!("Extracted idempotency key: {}", key.key);

    let request_hash = calculate_request_hash(parts.method.as_str(), parts.uri.path(), &body_bytes);

    // Rebuild the request because body was consumed
    let request = Request::from_parts(parts.clone(), Body::from(body_bytes.clone()));

    // 2. Run the actual handler
    let response = next.run(request).await;
    let response_status = response.status();

    let request = IdempotencyKeyForCreateRequest {
        key: key.key.clone(),
        request_hash: request_hash.clone(),
        endpoint: parts.uri.to_string(),
        state: "completed".to_string(),
        expires_at: Some((Utc::now() + Duration::seconds(10)).naive_utc()),
        response_status: response_status.as_u16() as i32,
    };

    debug!("Constructed idempotency request: {:?}", request);
    let create = IdempotencyService::create_idempotency_key(request).await;
    match create {
        Ok(_) => debug!("Idempotency key stored successfully"),
        Err(e) => debug!("Failed to store idempotency key: {}", e),
    };

    let mut response = response.into_response();
    response.headers_mut().insert(
        "X-Idempotency-Key",
        HeaderValue::from_str(&key.key).unwrap_or(HeaderValue::from_static("")),
    );
    response.headers_mut().insert(
        "X-Request-Hash",
        HeaderValue::from_str(&request_hash).unwrap_or(HeaderValue::from_static("")),
    );
    response
}
