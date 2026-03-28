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

static CACHE_KEY_PREFIX: &str = "idempotency_key:";
static CACHE_KEY_TTL: Duration = Duration::from_secs(10);

impl IdempotencyKey {
    pub fn value(&self) -> &str {
        &self.key
    }

    fn parse_user_id_from_baggage(value: &str) -> Option<&str> {
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
            method.as_str(),
            uri.path(),
            uri.query().unwrap_or(""),
            user_id.unwrap_or("")
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
    C: Clone + DeserializeOwned + Serialize + Default + Sync,
{
    type Rejection = IdempotencyKeyRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<T, C>,
    ) -> Result<Self, Self::Rejection> {
        let method = parts.method.clone();
        let uri = parts.uri.clone();

        let x_key = parts
            .headers
            .get("x-idempotency-key")
            .and_then(|v| v.to_str().ok())
            .and_then(IdempotencyKey::from_client_header);

        if let Some(key) = x_key {
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
                    if let Err(e) = state
                        .cache
                        .insert(cache_key, C::default(), Some(CACHE_KEY_TTL))
                    {
                        debug!("Failed to cache idempotency key: {:?}", e);
                        // Continue anyway - don't fail the request due to cache issues
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

        let user_id = parts
            .headers
            .get("baggage")
            .and_then(|v| v.to_str().ok())
            .and_then(IdempotencyKey::parse_user_id_from_baggage)
            .or_else(|| parts.headers.get("x-user-id").and_then(|v| v.to_str().ok()));

        let key = IdempotencyKey::deterministic(&method, &uri, user_id);

        // Check if deterministic key already exists in cache
        let cache_key = format!("{}{}", CACHE_KEY_PREFIX, key.key);
        match state.cache.get(&cache_key) {
            Ok(Some(_)) => {
                debug!("Deterministic idempotency key already used: {}", key.key);
                return Err(IdempotencyKeyRejection::new(key.key));
            }
            Ok(None) => {
                // Key not in cache, save it for 5 seconds
                if let Err(e) = state
                    .cache
                    .insert(cache_key, C::default(), Some(CACHE_KEY_TTL))
                {
                    debug!("Failed to cache deterministic idempotency key: {:?}", e);
                    // Continue anyway - don't fail the request due to cache issues
                }
            }
            Err(e) => {
                debug!(
                    "Failed to check deterministic idempotency key in cache: {:?}",
                    e
                );
                // Continue anyway - don't fail the request due to cache issues
            }
        }

        debug!("Generated deterministic idempotency key: {}", key.key);
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Uri};
    use http::{HeaderValue, Request};
    use shared_shared_app::state::AppState;
    use shared_shared_data_cache::cache::Cache;

    fn set_up_state() -> AppState<(), ()> {
        let cache = Cache::new("redis://127.0.0.1/", "idempotency_test").unwrap();
        AppState::new("idempotency_test".to_string(), cache, None)
    }

    #[test]
    fn test_from_client_header() {
        let key = IdempotencyKey::from_client_header("abc123").unwrap();
        assert_eq!(key.key, "abc123");
        assert_eq!(key.source, IdempotencyKeySource::ClientProvided);
    }

    #[test]
    fn test_from_client_header_empty() {
        assert!(IdempotencyKey::from_client_header("   ").is_none());
    }

    #[test]
    fn test_parse_user_id_from_baggage() {
        let baggage = "accesses=BAKERY_SUPPORT*A_ACCESS_KEY|EMAIL_NOTIFICATION_SALE*B_ACCESS_KEY,user_id=066df7b0-dcd1-4e7c-94a1-9b5f68794ca7";
        let user_id = IdempotencyKey::parse_user_id_from_baggage(baggage);
        assert_eq!(user_id, Some("066df7b0-dcd1-4e7c-94a1-9b5f68794ca7"));
    }

    #[test]
    fn test_deterministic_with_user_id() {
        let method = Method::POST;
        let uri: Uri = "/wallets/1/top-ups?foo=bar".parse().unwrap();
        let key1 = IdempotencyKey::deterministic(&method, &uri, Some("user-1"));
        let key2 = IdempotencyKey::deterministic(&method, &uri, Some("user-1"));
        assert_eq!(key1.key, key2.key);
        assert_eq!(key1.source, IdempotencyKeySource::Deterministic);
    }

    #[tokio::test]
    async fn test_from_request_parts_client_key() {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/wallets/1/top-ups")
            .body(())
            .unwrap();

        let mut parts = req.into_parts().0;
        parts
            .headers
            .insert("x-idempotency-key", HeaderValue::from_static("abc123"));

        let state = set_up_state();
        let id = IdempotencyKey::from_request_parts(&mut parts, &state)
            .await
            .unwrap();
        assert_eq!(id.key, "abc123");
        assert_eq!(id.source, IdempotencyKeySource::ClientProvided);
    }

    #[tokio::test]
    async fn test_from_request_parts_baggage_user_id() {
        let req = Request::builder()
            .method(Method::POST)
            .uri("/wallets/1/top-ups")
            .body(())
            .unwrap();

        let mut parts = req.into_parts().0;
        parts.headers.insert(
            "baggage",
            HeaderValue::from_static("accesses=XYZ,user_id=066df7b0-dcd1-4e7c-94a1-9b5f68794ca7"),
        );

        let state = set_up_state();
        let id = IdempotencyKey::from_request_parts(&mut parts, &state)
            .await
            .unwrap();
        assert_eq!(id.source, IdempotencyKeySource::Deterministic);
        assert!(!id.key.is_empty());
    }
}
