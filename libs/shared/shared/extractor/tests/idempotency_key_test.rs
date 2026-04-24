use axum::extract::FromRequestParts;
use axum::http::{Method, Uri};
use http::{HeaderValue, Request};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use shared_shared_app::state::AppState;
use shared_shared_data_cache::cache::Cache;
use shared_shared_extractor::{IdempotencyCacheType, IdempotencyKey, IdempotencyKeySource};

#[derive(Clone, Default, Serialize, Deserialize)]
struct TestCache;

impl IdempotencyCacheType for TestCache {
    fn default_idempotency_value() -> Self {
        TestCache
    }
}

fn set_up_state() -> AppState<(), TestCache> {
    let cache = Cache::new("redis://127.0.0.1/", "idempotency_test").unwrap();
    let db = DatabaseConnection::default();
    AppState::new(&db, cache, None)
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
