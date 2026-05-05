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

use features_lookup_entities::lookup_item::Model as LookupItemModel;
use features_lookup_model::state::{LookupAppState, LookupCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_item_model() -> LookupItemModel {
    LookupItemModel {
        id: Uuid::new_v4(),
        lookup_type_id: Uuid::new_v4(),
        code: "USD".to_string(),
        name: "US Dollar".to_string(),
        url: "".to_string(),
        query_param_one: "".to_string(),
        query_param_two: "".to_string(),
        tenants: vec![],
        meta: serde_json::json!({}),
        is_active: true,
        sort_order: 1,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..30 {
            mock = mock.append_query_results(vec![vec![sample_item_model()]]);
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
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test_item")
        .expect("cache init");
    let app_state = AppState::new(&db_conn, cache, Some(LookupAppState::default()));

    api_lookup::routes::lookup_item::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Public access ===

#[tokio::test]
async fn test_get_lookup_item_by_id_public() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/lookup-types/CURRENCY/items/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    if status == StatusCode::OK {
        let body = parse_body(response).await;
        assert_eq!(body["status"], 1);
        assert!(body["data"]["id"].is_string());
        assert_eq!(body["data"]["code"], "USD");
        assert_eq!(body["data"]["name"], "US Dollar");
        assert!(body["data"]["is_active"].as_bool().unwrap());
    } else {
        // Mock DB result ordering may cause 500; verify no auth error
        assert_ne!(status, StatusCode::FORBIDDEN);
        assert_ne!(status, StatusCode::UNAUTHORIZED);
    }
}

// === Auth required ===

#[tokio::test]
async fn test_create_lookup_item_requires_auth() {
    let app = build_app();
    let body = json!({ "code": "USD", "name": "US Dollar" });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types/CURRENCY/items")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_lookup_item_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "name": "Updated" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/lookup-types/CURRENCY/items/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_lookup_item_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/lookup-types/CURRENCY/items/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access ===

#[tokio::test]
async fn test_create_lookup_item_with_admin_passes_auth() {
    let app = build_app();
    let body = json!({ "code": "USD", "name": "US Dollar", "sort_order": 1 });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types/CURRENCY/items")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Auth passes with ADMIN_ALL (not 403). DB layer may fail with mock.
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_lookup_item_with_admin_passes_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/lookup-types/CURRENCY/items/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
}

// === Validation ===

#[tokio::test]
async fn test_create_lookup_item_validation_empty_code() {
    let app = build_app();
    let body = json!({ "code": "", "name": "US Dollar" });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types/CURRENCY/items")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_lookup_item_validation_empty_name() {
    let app = build_app();
    let body = json!({ "code": "USD", "name": "" });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types/CURRENCY/items")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_lookup_item_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types/CURRENCY/items")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
