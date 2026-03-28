use axum::extract::FromRequestParts;
use axum::http::{request::Parts, uri::Uri, Method, StatusCode};
use sha2::{Digest, Sha256};
use tracing::debug;

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
pub struct IdempotencyKeyRejection;

impl axum::response::IntoResponse for IdempotencyKeyRejection {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            "Missing or invalid idempotency key",
        )
            .into_response()
    }
}

impl<S> FromRequestParts<S> for IdempotencyKey
where
    S: Send + Sync,
{
    type Rejection = IdempotencyKeyRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let method = parts.method.clone();
        let uri = parts.uri.clone();

        let x_key = parts
            .headers
            .get("x-idempotency-key")
            .and_then(|v| v.to_str().ok())
            .and_then(IdempotencyKey::from_client_header);

        if let Some(key) = x_key {
            // TODO save key.value into redis with short expiration to prevent replay attack
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
        debug!("Generated deterministic idempotency key: {}", key.key);
        // TODO save key.value into redis with short expiration to prevent replay attack
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Uri};
    use http::{HeaderValue, Request};

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

        let id = IdempotencyKey::from_request_parts(&mut parts, &())
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

        let id = IdempotencyKey::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(id.source, IdempotencyKeySource::Deterministic);
        assert!(!id.key.is_empty());
    }
}
