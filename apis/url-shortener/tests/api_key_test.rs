use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware,
    Router,
};
use chrono::Utc;
use http::header;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::json;
use std::sync::{Arc, Once};
use tower::ServiceExt;
use uuid::Uuid;

use shared_shared_app::{mapper::main_response_mapper, state::AppState};
use shared_shared_config::db::{DB_READ, DB_WRITE};
use shared_shared_data_cache::cache::Cache;

use features_url_shortener_entities::api_key::Model as ApiKeyModel;
use features_url_shortener_model::state::{UrlShortenerAppState, UrlShortenerCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=default";

static INIT: Once = Once::new();

fn sample_api_key_model() -> ApiKeyModel {
    ApiKeyModel {
        id: Uuid::new_v4(),
        user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
        key_hash: "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1".to_string(),
        name: "Test Key".to_string(),
        is_active: true,
        last_used_at: None,
        created_at: Utc::now().naive_utc(),
    }
}

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..20 {
            mock = mock.append_query_results(vec![vec![sample_api_key_model()]]);
        }
        mock = mock.append_exec_results(vec![
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
        ]);
        let conn = Arc::new(mock.into_connection());
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app() -> Router {
    init_mock_db();

    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache =
        Cache::<String, UrlShortenerCacheState>::new("redis://127.0.0.1/", "test_api_keys")
            .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(UrlShortenerAppState::default()));

    api_url_shortener::routes::api_key::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

// === Auth required tests ===

#[tokio::test]
async fn test_create_api_key_requires_auth() {
    let app = build_app();

    let body = json!({
        "name": "My Key"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/api-keys")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_api_keys_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/api-keys")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_api_key_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/api-keys/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

// === Admin access tests ===

#[tokio::test]
async fn test_create_api_key_with_admin() {
    let app = build_app();

    let body = json!({
        "name": "My Integration Key"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/api-keys")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    assert_ne!(status, StatusCode::FORBIDDEN);
    assert_ne!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_api_keys_with_admin() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/api-keys")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    assert_ne!(status, StatusCode::FORBIDDEN);
    assert_ne!(status, StatusCode::UNAUTHORIZED);
}

// === Validation tests ===

#[tokio::test]
async fn test_create_api_key_empty_name() {
    let app = build_app();

    let body = json!({
        "name": ""
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/api-keys")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Empty name fails validation (min length = 1)
    assert_ne!(response.status(), StatusCode::OK);
}
