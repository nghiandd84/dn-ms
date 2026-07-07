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

use features_email_template_entities::email_templates::Model as EmailTemplateModel;
use features_email_template_model::state::EmailTemplateCacheState;

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> EmailTemplateModel {
    EmailTemplateModel {
        id: 1,
        name: "Welcome Email".to_string(),
        description: "Welcome email template".to_string(),
        key: "welcome_email".to_string(),
        is_active: true,
        user_id: Uuid::new_v4(),
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
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
            MockExecResult { last_insert_id: 1, rows_affected: 1 },
        ]);
        let conn = Arc::new(mock.into_connection());
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app() -> Router {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, ()>::new("redis://127.0.0.1/", "test_email_template")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(EmailTemplateCacheState::default()));

    api_email_template::routes::email_template::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

#[allow(dead_code)]
async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Auth required tests ===

#[tokio::test]
async fn test_create_email_template_requires_auth() {
    let app = build_app();

    let body = json!({
        "name": "Welcome Email",
        "key": "welcome_email"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/email-templates")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_email_template_requires_auth() {
    let app = build_app();

    let body = json!({ "name": "Updated" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri("/email-templates/1")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_email_template_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri("/email-templates/1")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_get_email_template_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/email-templates/1")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_filter_email_templates_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/email-templates")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access tests ===

#[tokio::test]
async fn test_create_email_template_with_admin() {
    let app = build_app();

    let body = json!({
        "name": "Welcome Email",
        "key": "welcome_email",
        "user_id": "00000000-0000-0000-0000-000000000000"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/email-templates")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_email_template_with_admin() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri("/email-templates/1")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
}

// === Validation tests ===

#[tokio::test]
async fn test_create_email_template_validation_short_name() {
    let app = build_app();

    // name min length is 2
    let body = json!({
        "name": "X",
        "key": "welcome_email"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/email-templates")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_email_template_validation_short_key() {
    let app = build_app();

    // key min length is 2
    let body = json!({
        "name": "Welcome Email",
        "key": "X"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/email-templates")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_email_template_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/email-templates")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
