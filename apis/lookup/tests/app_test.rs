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

use features_lookup_model::state::{LookupAppState, LookupCacheState};

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

fn build_app_state() -> AppState<LookupAppState, LookupCacheState> {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test")
        .expect("cache creation should not connect");
    AppState::new(&db_conn, cache, Some(LookupAppState::default()))
}

#[test]
fn test_app_config() {
    let config = AppConfig::new("LOOKUP".to_string(), Some("lookup".to_string()), true, true);

    assert_eq!(config.app_key, "LOOKUP");
    assert_eq!(config.db_config.db_scheme, Some("lookup".to_string()));
    assert!(config.has_swagger);
    assert!(config.has_discovery_service);
}

#[tokio::test]
async fn test_routes_lookup_type_registered() {
    let app_state = build_app_state();
    let router: Router = api_lookup::routes::lookup_type::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_lookup_item_registered() {
    let app_state = build_app_state();
    let router: Router = api_lookup::routes::lookup_item::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/CURRENCY/items")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_lookup_item_translation_registered() {
    let app_state = build_app_state();
    let router: Router = api_lookup::routes::lookup_item_translation::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/CURRENCY/items/00000000-0000-0000-0000-000000000000/translations")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_unknown_route_returns_not_found() {
    let app_state = build_app_state();
    let router: Router = api_lookup::routes::lookup_type::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
