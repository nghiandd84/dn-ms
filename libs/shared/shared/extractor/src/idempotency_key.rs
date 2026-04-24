use axum::extract::FromRequestParts;
use axum::http::{request::Parts, uri::Uri, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::time::Duration;
use tracing::{debug, error};

use shared_shared_app::state::AppState;
use shared_shared_data_error::app::AppError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdempotencyKeySource {
    ClientProvided,
    Deterministic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyKey {
    pub key: String,
    pub source: IdempotencyKeySource,
}

static CACHE_KEY_PREFIX: &str = "idempotency:";
static CACHE_KEY_TTL: Duration = Duration::from_secs(10); // 10 seconds

impl IdempotencyKey {
    pub fn value(&self) -> &str {
        &self.key
    }

    pub fn parse_user_id_from_baggage(value: &str) -> Option<&str> {
        value
            .split(',')
            .map(str::trim)
            .filter_map(|kv| {
                let mut pieces = kv.splitn(2, '=');
                let key = pieces.next()?.trim();
                let val = pieces.next()?.trim();
                if key == "user_id" || key == "user-id" {
                    Some(val)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn from_client_header(header_value: &str) -> Option<Self> {
        let trimmed = header_value.trim();
        if trimmed.is_empty() {
            return None;
        }

        Some(Self {
            key: trimmed.to_owned(),
            source: IdempotencyKeySource::ClientProvided,
        })
    }

    pub fn deterministic(method: &Method, uri: &Uri, user_id: Option<&str>) -> Self {
        let normalized_key = format!(
            "{}|{}|{}|{}",
            user_id.unwrap_or("anonymous"),
            method.as_str(),
            uri.path(),
            uri.query().unwrap_or(""),
        );
        tracing::debug!("Normalized key for idempotency: {}", normalized_key);
        let mut hasher = Sha256::new();
        hasher.update(normalized_key.as_bytes());
        let result = hasher.finalize();
        let key = format!("{:x}", result);

        IdempotencyKey {
            key,
            source: IdempotencyKeySource::Deterministic,
        }
    }

    pub fn get_idempotency_key(parts: &Parts) -> Self {
        let method = parts.method.clone();
        let uri = parts.uri.clone();

        let x_key = parts
            .headers
            .get("x-idempotency-key")
            .and_then(|v| v.to_str().ok())
            .and_then(IdempotencyKey::from_client_header);

        if let Some(key) = x_key {
            debug!("Using client-provided idempotency key: {}", key.key);
            return key;
        }

        let user_id = parts
            .headers
            .get("baggage")
            .and_then(|v| v.to_str().ok())
            .and_then(IdempotencyKey::parse_user_id_from_baggage)
            .or_else(|| parts.headers.get("x-user-id").and_then(|v| v.to_str().ok()));

        let key = IdempotencyKey::deterministic(&method, &uri, user_id);
        debug!("Generated deterministic idempotency key: {}", key.key);
        key
    }
}
pub trait IdempotencyCacheType {
    fn default_idempotency_value() -> Self;
}

#[derive(Debug)]
pub struct IdempotencyKeyRejection {
    pub key: String,
}

impl IdempotencyKeyRejection {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl IntoResponse for IdempotencyKeyRejection {
    fn into_response(self) -> Response {
        error!("Idempotency error: {:?}", self);
        (
            StatusCode::BAD_REQUEST,
            AppError::DuplicateEntry("Missing or invalid idempotency key".to_string()),
        )
            .into_response()
    }
}

impl<T, C> FromRequestParts<AppState<T, C>> for IdempotencyKey
where
    T: Clone + Sync,
    C: Clone + Serialize + DeserializeOwned + Default + Sync + IdempotencyCacheType,
{
    type Rejection = IdempotencyKeyRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<T, C>,
    ) -> Result<Self, Self::Rejection> {
        let key = IdempotencyKey::get_idempotency_key(parts);

        // Check if key already exists in cache (replay attack prevention)
        let cache_key = format!("{}{}", CACHE_KEY_PREFIX, key.key);
        match state.cache.get(&cache_key) {
            Ok(Some(_)) => {
                debug!(
                    "Idempotency key already used (replay attack detected): {}",
                    key.key
                );
                return Err(IdempotencyKeyRejection::new(key.key));
            }
            Ok(None) => {
                // Key not in cache, save it for 5 seconds
                if let Err(e) = state.cache.insert(
                    cache_key,
                    C::default_idempotency_value(),
                    Some(CACHE_KEY_TTL),
                ) {
                    debug!("Failed to cache idempotency key: {:?}", e);
                    // Continue anyway - don't fail the request due to cache issues
                } else {
                    debug!("Cached client-provided idempotency key: {}", key.key);
                    // TODO: Save idempotency key and response in persistent store for longer-term caching and retrieval
                }
            }
            Err(e) => {
                debug!("Failed to check idempotency key in cache: {:?}", e);
                // Continue anyway - don't fail the request due to cache issues
            }
        }

        debug!("Using client-provided idempotency key: {}", key.key);
        return Ok(key);
    }
}
