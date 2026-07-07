use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::sync::{Arc, Once};
use tower::ServiceExt;

use shared_shared_app::{config::AppConfig, state::AppState};
use shared_shared_config::db::{DB_READ, DB_WRITE};
use shared_shared_data_cache::cache::Cache;

use features_auth_model::state::{AuthAppState, AuthCacheState};

static INIT: Once = Once::new();

fn init_mock_db() {
    INIT.call_once(|| {
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![
                MockExecResult { last_insert_id: 0, rows_affected: 0 },
            ])
            .into_connection();
        let conn = Arc::new(mock_db);
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app_state() -> AppState<AuthAppState, AuthCacheState> {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, AuthCacheState>::new("redis://127.0.0.1/", "test")
        .expect("cache creation should not connect");
    AppState::new(&db_conn, cache, Some(AuthAppState::default()))
}

#[test]
fn test_app_config() {
    let config = AppConfig::new("AUTH".to_string(), Some("auth".to_string()), true, true);

    assert_eq!(config.app_key, "AUTH");
    assert_eq!(config.db_config.db_scheme, Some("auth".to_string()));
    assert!(config.has_swagger);
    assert!(config.has_discovery_service);
}

// === Route registration tests ===

#[tokio::test]
async fn test_routes_role_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::role::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/roles")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_client_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::client::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/clients")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_scope_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::scope::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/scopes")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_permission_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::permission::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/permissions")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_user_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::user::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/users")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_auth_code_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::auth_code::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/auth-codes")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_token_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::token::routes(&app_state);

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/tokens/oauth")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_authentication_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::authentication::routes(&app_state);

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/requests/code")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_signup_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::signup::routes(&app_state);

    let req = Request::builder()
        .method(Method::POST)
        .uri("/public/signup/active")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_active_code_registered() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::active_code::routes(&app_state);

    let req = Request::builder()
        .method(Method::POST)
        .uri("/internal/active-codes/mark-sent")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_unknown_route_returns_not_found() {
    let app_state = build_app_state();
    let router: Router = api_auth::routes::role::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
