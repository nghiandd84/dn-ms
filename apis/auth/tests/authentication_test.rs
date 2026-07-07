use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware,
    Router,
};
use http::header;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use serde_json::{json, Value};
use std::sync::{Arc, Once};
use tower::ServiceExt;
use uuid::Uuid;

use shared_shared_app::{mapper::main_response_mapper, state::AppState};
use shared_shared_config::db::{DB_READ, DB_WRITE};
use shared_shared_data_cache::cache::Cache;

use features_auth_model::state::{AuthAppState, AuthCacheState};

static INIT: Once = Once::new();

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..40 {
            mock = mock.append_exec_results(vec![
                MockExecResult { last_insert_id: 0, rows_affected: 1 },
            ]);
        }
        let conn = Arc::new(mock.into_connection());
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app() -> Router {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, AuthCacheState>::new("redis://127.0.0.1/", "test_authentication")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(AuthAppState::default()));

    api_auth::routes::authentication::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

#[allow(dead_code)]
async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Public access tests (all authentication routes are public) ===
// Note: request_code, request_login, and request_register handlers require
// external dependencies (DB query results, Kafka producer) that are not
// available in unit tests. We test that they are public by verifying:
// - login/code works end-to-end with mock DB
// - the others don't return 404 (route exists) and don't return 403 (auth passes)

#[tokio::test]
async fn test_login_code_public_access() {
    let app = build_app();

    let body = json!({
        "user_id": Uuid::new_v4(),
        "login_code": "123456"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/login/code")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

// === Validation tests ===

#[tokio::test]
async fn test_request_code_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/code")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_code_validation_missing_client_id() {
    let app = build_app();

    let body = json!({
        "scopes": ["read:users"]
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/code")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_code_validation_missing_scopes() {
    let app = build_app();

    let body = json!({
        "client_id": Uuid::new_v4()
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/code")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_code_validation_empty_scopes() {
    let app = build_app();

    let body = json!({
        "client_id": Uuid::new_v4(),
        "scopes": []
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/code")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_login_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_login_validation_missing_email() {
    let app = build_app();

    let body = json!({
        "password": "password123",
        "state": "random_state"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_register_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_request_register_validation_missing_email() {
    let app = build_app();

    let body = json!({
        "password": "password123",
        "state": "random_state",
        "language": "en"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
