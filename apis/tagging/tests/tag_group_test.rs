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

use features_tagging_entities::tag_group::Model as TagGroupModel;
use features_tagging_model::state::{TaggingAppState, TaggingCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> TagGroupModel {
    TagGroupModel {
        id: Uuid::new_v4(),
        tenant_id: "test-tenant".to_string(),
        code: "GENRE".to_string(),
        name: "Genre".to_string(),
        description: "Music genres".to_string(),
        parent_id: None,
        is_active: true,
        sort_order: 1,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        children: vec![],
        parent: vec![],
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
    let cache = Cache::<String, TaggingCacheState>::new("redis://127.0.0.1/", "test_tagging")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(TaggingAppState::default()));

    api_tagging::routes::tag_group::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_get_tag_groups_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tag-groups")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Without baggage, Auth extractor should reject
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tag_groups_with_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tag-groups")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tag_group_by_id() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/tag-groups/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert!(body["data"]["id"].is_string());
    assert_eq!(body["data"]["code"], "GENRE");
    assert_eq!(body["data"]["name"], "Genre");
}

#[tokio::test]
async fn test_create_tag_group_requires_auth() {
    let app = build_app();

    let body = json!({
        "code": "GENRE",
        "name": "Genre",
        "description": "Music genres"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/tag-groups")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_tag_group_with_auth() {
    let app = build_app();

    let body = json!({
        "code": "GENRE",
        "name": "Genre",
        "description": "Music genres",
        "sort_order": 1
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/tag-groups")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert!(body["data"]["ok"].as_bool().unwrap());
    assert!(body["data"]["id"].is_string());
}

#[tokio::test]
async fn test_update_tag_group() {
    let app = build_app();
    let id = Uuid::new_v4();

    let body = json!({
        "name": "Updated Genre",
        "description": "Updated description"
    });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/tag-groups/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_tag_group() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/tag-groups/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
