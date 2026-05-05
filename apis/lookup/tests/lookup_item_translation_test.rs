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

use features_lookup_entities::lookup_item_translation::Model as TranslationModel;
use features_lookup_model::state::{LookupAppState, LookupCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_translation_model() -> TranslationModel {
    TranslationModel {
        id: Uuid::new_v4(),
        lookup_item_id: Uuid::new_v4(),
        locale: "en-US".to_string(),
        name: "US Dollar".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..30 {
            mock = mock.append_query_results(vec![vec![sample_translation_model()]]);
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
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test_trans")
        .expect("cache init");
    let app_state = AppState::new(&db_conn, cache, Some(LookupAppState::default()));

    api_lookup::routes::lookup_item_translation::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

fn base_path(item_id: &Uuid) -> String {
    format!("/lookup-types/CURRENCY/items/{}/translations", item_id)
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Auth required ===

#[tokio::test]
async fn test_create_translation_requires_auth() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let body = json!({ "locale": "vi", "name": "Đô la Mỹ" });

    let req = Request::builder()
        .method(Method::POST)
        .uri(base_path(&item_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_translation_requires_auth() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let trans_id = Uuid::new_v4();
    let body = json!({ "name": "Updated" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("{}/{}", base_path(&item_id), trans_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_translation_requires_auth() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let trans_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("{}/{}", base_path(&item_id), trans_id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access ===

#[tokio::test]
async fn test_create_translation_with_admin() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let body = json!({ "locale": "vi", "name": "Đô la Mỹ" });

    let req = Request::builder()
        .method(Method::POST)
        .uri(base_path(&item_id))
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
async fn test_delete_translation_with_admin() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let trans_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("{}/{}", base_path(&item_id), trans_id))
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

// === Validation ===

#[tokio::test]
async fn test_create_translation_validation_empty_locale() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let body = json!({ "locale": "", "name": "Đô la Mỹ" });

    let req = Request::builder()
        .method(Method::POST)
        .uri(base_path(&item_id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_translation_validation_empty_name() {
    let app = build_app();
    let item_id = Uuid::new_v4();
    let body = json!({ "locale": "vi", "name": "" });

    let req = Request::builder()
        .method(Method::POST)
        .uri(base_path(&item_id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_translation_invalid_json() {
    let app = build_app();
    let item_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::POST)
        .uri(base_path(&item_id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("invalid"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
