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

use features_lookup_entities::lookup_type::Model as LookupTypeModel;
use features_lookup_model::state::{LookupAppState, LookupCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> LookupTypeModel {
    LookupTypeModel {
        id: Uuid::new_v4(),
        tenant_id: "test-tenant".to_string(),
        code: "CURRENCY".to_string(),
        name: "Currency Types".to_string(),
        description: "All currency types".to_string(),
        is_active: true,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        items: vec![],
    }
}

fn init_mock_db() {
    INIT.call_once(|| {
        // Provide enough results for all tests running in parallel.
        // The paginated list query uses 2 DB calls (count + data).
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
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test_lookup")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(LookupAppState::default()));

    api_lookup::routes::lookup_type::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Public access tests ===

#[tokio::test]
async fn test_get_lookup_types_public_access() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Public endpoint via PublicAccess extractor - should not return auth errors (403)
    let status = response.status();
    assert_ne!(status, StatusCode::FORBIDDEN);
    assert_ne!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_lookup_type_by_id_public_access() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/lookup-types/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert!(body["data"]["id"].is_string());
    assert_eq!(body["data"]["code"], "CURRENCY");
    assert_eq!(body["data"]["name"], "Currency Types");
    assert!(body["data"]["is_active"].as_bool().unwrap());
}

// === Auth required tests ===

#[tokio::test]
async fn test_create_lookup_type_requires_auth() {
    let app = build_app();

    let body = json!({
        "code": "CURRENCY",
        "name": "Currency Types",
        "description": "All currency types"
    });

    // No baggage header - Auth extractor rejects
    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_lookup_type_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let body = json!({ "description": "Updated" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/lookup-types/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_lookup_type_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/lookup-types/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

// === Admin access tests ===

#[tokio::test]
async fn test_create_lookup_type_with_admin() {
    let app = build_app();

    let body = json!({
        "code": "CURRENCY",
        "name": "Currency Types",
        "description": "All currency types"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert_eq!(body["data"]["ok"], true);
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn test_delete_lookup_type_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/lookup-types/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert_eq!(body["data"]["ok"], true);
    assert!(body["data"]["id"].is_string());
}

// === Validation tests ===

#[tokio::test]
async fn test_create_lookup_type_validation_empty_code() {
    let app = build_app();

    let body = json!({
        "code": "",
        "name": "Currency Types"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_lookup_type_validation_empty_name() {
    let app = build_app();

    let body = json!({
        "code": "VALID_CODE",
        "name": ""
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_lookup_type_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
