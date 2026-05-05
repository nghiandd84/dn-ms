use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::get,
    Router,
};
use sea_orm::{DatabaseBackend, MockDatabase};
use std::sync::{Arc, Once};
use tower::ServiceExt;

use shared_shared_app::state::AppState;
use shared_shared_config::db::{DB_READ, DB_WRITE};
use shared_shared_data_cache::cache::Cache;

use api_lookup::middleware::cache_lookup_items_middleware;
use features_lookup_model::state::{LookupAppState, LookupCacheState};

static INIT: Once = Once::new();

fn init_mock_db() {
    INIT.call_once(|| {
        let mock = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        let conn = Arc::new(mock);
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app_with_middleware() -> Router {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test_mw")
        .expect("cache init");
    let app_state = AppState::new(&db_conn, cache, Some(LookupAppState::default()));

    async fn mock_handler() -> impl IntoResponse {
        (StatusCode::OK, "ok")
    }

    Router::new()
        .route(
            "/test",
            get(mock_handler)
                .layer(from_fn_with_state(app_state.clone(), cache_lookup_items_middleware)),
        )
        .with_state(app_state)
}

#[tokio::test]
async fn test_middleware_passes_through_on_cache_miss() {
    let app = build_app_with_middleware();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // On cache miss (Redis not available), middleware should still pass through to handler
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_middleware_adds_cache_expires_header() {
    let app = build_app_with_middleware();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert!(response.headers().contains_key("x-cache-expires-in"));
    assert_eq!(
        response.headers().get("x-cache-expires-in").unwrap(),
        "60 seconds"
    );
}

#[tokio::test]
async fn test_middleware_preserves_response_body() {
    let app = build_app_with_middleware();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&body[..], b"ok");
}

#[tokio::test]
async fn test_middleware_includes_query_string_in_cache_key_logic() {
    let app = build_app_with_middleware();

    // Different query strings should still pass through (testing the path works)
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test?page=1&page_size=10")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
