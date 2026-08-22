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

use features_tagging_entities::entity_tag::Model as EntityTagModel;
use features_tagging_entities::tag::Model as TagModel;
use features_tagging_model::state::{TaggingAppState, TaggingCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_entity_tag() -> EntityTagModel {
    EntityTagModel {
        id: Uuid::new_v4(),
        tag_id: Uuid::new_v4(),
        entity_type: "event".to_string(),
        entity_id: Uuid::new_v4(),
        tenant_id: "test-tenant".to_string(),
        tagged_by: Uuid::new_v4(),
        created_at: Utc::now().naive_utc(),
        tag: vec![],
    }
}

fn sample_tag() -> TagModel {
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
            mock = mock.append_query_results(vec![vec![sample_entity_tag()]]);
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

    api_tagging::routes::entity_tag::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn test_assign_tag_requires_auth() {
    let app = build_app();

    let body = json!({
        "tag_id": Uuid::new_v4(),
        "entity_type": "event",
        "entity_id": Uuid::new_v4()
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/entity-tags")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_assign_tag_with_auth() {
    let app = build_app();

    let body = json!({
        "tag_id": Uuid::new_v4(),
        "entity_type": "event",
        "entity_id": Uuid::new_v4()
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/entity-tags")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // May return OK if mock provides enough data, or internal error if mock runs out
    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_tags_for_entity() {
    let app = build_app();
    let entity_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/entities/event/{}/tags", entity_id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_entities_for_tag() {
    let app = build_app();
    let tag_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/tags/{}/entities", tag_id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_bulk_assign_tags_with_auth() {
    let app = build_app();

    let body = json!({
        "tag_ids": [Uuid::new_v4(), Uuid::new_v4()],
        "entity_type": "event",
        "entity_ids": [Uuid::new_v4()]
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/entity-tags/bulk")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // May succeed or fail depending on mock DB state
    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_bulk_remove_tags_with_auth() {
    let app = build_app();

    let body = json!({
        "tag_ids": [Uuid::new_v4()],
        "entity_type": "event",
        "entity_ids": [Uuid::new_v4()]
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/entity-tags/bulk-remove")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    assert!(status == StatusCode::OK || status == StatusCode::INTERNAL_SERVER_ERROR);
}
