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

use features_tagging_entities::tag::Model as TagModel;
use features_tagging_model::state::{TaggingAppState, TaggingCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> TagModel {
    TagModel {
        id: Uuid::new_v4(),
        tenant_id: "test-tenant".to_string(),
        tag_group_id: Uuid::new_v4(),
        name: "Rock".to_string(),
        slug: "rock".to_string(),
        color: "#FF5733".to_string(),
        description: "Rock music genre".to_string(),
        alias_of: None,
        is_active: true,
        sort_order: 1,
        usage_count: 5,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        tag_group: vec![],
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

    api_tagging::routes::tag::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_get_tags_requires_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tags")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tags_with_auth() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tags")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tag_by_id() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/tags/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["status"], 1);
    assert!(body["data"]["id"].is_string());
    assert_eq!(body["data"]["name"], "Rock");
    assert_eq!(body["data"]["slug"], "rock");
    assert_eq!(body["data"]["color"], "#FF5733");
    assert_eq!(body["data"]["usage_count"], 5);
}

#[tokio::test]
async fn test_create_tag_with_auth() {
    let app = build_app();

    let body = json!({
        "tag_group_id": Uuid::new_v4(),
        "name": "Jazz",
        "slug": "jazz",
        "color": "#0000FF",
        "description": "Jazz music",
        "sort_order": 2
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/tags")
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
async fn test_search_tags() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tags/search?search=rock&limit=5")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_search_tags_empty_query() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/tags/search?search=")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    // Empty search returns empty array
    assert_eq!(body["status"], 1);
}

#[tokio::test]
async fn test_update_tag() {
    let app = build_app();
    let id = Uuid::new_v4();

    let body = json!({
        "name": "Classic Rock",
        "color": "#FF0000"
    });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/tags/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_tag() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/tags/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tag_usage() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/tags/{}/usage", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Should return 200 (uses mock DB count)
    assert_eq!(response.status(), StatusCode::OK);
}
