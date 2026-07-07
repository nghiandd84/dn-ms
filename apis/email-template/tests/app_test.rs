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

use features_email_template_model::state::EmailTemplateCacheState;

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

fn build_app_state() -> AppState<EmailTemplateCacheState> {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, ()>::new("redis://127.0.0.1/", "test")
        .expect("cache creation should not connect");
    AppState::new(&db_conn, cache, Some(EmailTemplateCacheState::default()))
}

#[test]
fn test_app_config() {
    let config = AppConfig::new(
        "EMAIL_TEMPLATE".to_string(),
        Some("email_template".to_string()),
        true,
        true,
    );

    assert_eq!(config.app_key, "EMAIL_TEMPLATE");
    assert_eq!(
        config.db_config.db_scheme,
        Some("email_template".to_string())
    );
    assert!(config.has_swagger);
    assert!(config.has_discovery_service);
}

// === Route registration tests ===

#[tokio::test]
async fn test_routes_email_template_registered() {
    let app_state = build_app_state();
    let router: Router = api_email_template::routes::email_template::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/email-templates")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_template_translation_registered() {
    let app_state = build_app_state();
    let router: Router = api_email_template::routes::template_translation::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/template-translations")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routes_template_placeholder_registered() {
    let app_state = build_app_state();
    let router: Router = api_email_template::routes::template_placeholder::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/template-placeholders")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_unknown_route_returns_not_found() {
    let app_state = build_app_state();
    let router: Router = api_email_template::routes::email_template::routes(&app_state);

    let req = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();
    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
