use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware,
    Router,
};
use chrono::Utc;
use http::header;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::{json, Value};
use std::sync::{Arc, Once};
use tower::ServiceExt;
use uuid::Uuid;

use shared_shared_app::{mapper::main_response_mapper, state::AppState};
use shared_shared_config::db::{DB_READ, DB_WRITE};
use shared_shared_data_cache::cache::Cache;

use features_url_shortener_entities::shortened_url::Model as ShortenedUrlModel;
use features_url_shortener_model::state::{UrlShortenerAppState, UrlShortenerCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=default";

static INIT: Once = Once::new();

fn sample_model() -> ShortenedUrlModel {
    ShortenedUrlModel {
        id: Uuid::new_v4(),
        user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
        original_url: "https://example.com/long-url".to_string(),
        short_code: "abc1234".to_string(),
        title: "Test Link".to_string(),
        is_active: true,
        expires_at: None,
        click_count: 42,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..40 {
            mock = mock.append_query_results(vec![vec![sample_model()]]);
        }
        mock = mock.append_exec_results(vec![
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
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
        Cache::<String, UrlShortenerCacheState>::new("redis://127.0.0.1/", "test_url_shortener")
            .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(UrlShortenerAppState::default()));

    api_url_shortener::routes::shortened_url::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Auth required tests ===

#[tokio::test]
async fn test_create_url_requires_auth() {
    let app = build_app();

    let body = json!({
        "original_url": "https://example.com/test"
    });

    // No baggage header - should be rejected
    let req = Request::builder()
        .method(Method::POST)
        .uri("/urls")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_urls_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/urls")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_url_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let body = json!({ "title": "Updated" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/urls/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_url_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/urls/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

// === Admin access tests ===

#[tokio::test]
async fn test_create_url_with_admin() {
    let app = build_app();

    let body = json!({
        "original_url": "https://example.com/test",
        "title": "Test URL"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/urls")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // With mock DB, creation should succeed or at least not be 403
    let status = response.status();
    assert_ne!(status, StatusCode::FORBIDDEN);
    assert_ne!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_url_by_id_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/urls/{}", id))
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
async fn test_create_url_invalid_url_format() {
    let app = build_app();

    let body = json!({
        "original_url": "not-a-valid-url"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/urls")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Should fail validation
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_url_custom_code_too_short() {
    let app = build_app();

    let body = json!({
        "original_url": "https://example.com",
        "custom_code": "ab"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/urls")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // custom_code min length is 3
    assert_ne!(response.status(), StatusCode::OK);
}
