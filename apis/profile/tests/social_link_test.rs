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

use features_profiles_entities::social_link::Model as SocialLinkModel;
use features_profiles_model::state::{ProfileAppState, ProfileCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> SocialLinkModel {
    SocialLinkModel {
        id: Uuid::new_v4(),
        profile_id: Uuid::new_v4(),
        platform: "github".to_string(),
        url: "https://github.com/johndoe".to_string(),
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
    let cache = Cache::<String, ProfileCacheState>::new("redis://127.0.0.1/", "test_social")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(ProfileAppState::default()));

    api_profile::routes::social_link::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === Auth required ===

#[tokio::test]
async fn test_create_social_link_requires_auth() {
    let app = build_app();
    let body = json!({
        "profile_id": Uuid::new_v4(),
        "platform": "github",
        "url": "https://github.com/johndoe"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/social-links")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_social_link_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "url": "https://github.com/janedoe" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/social-links/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_social_link_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/social-links/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access ===

#[tokio::test]
async fn test_get_social_links_with_admin() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/social-links")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_social_link_by_id_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/social-links/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_social_links_by_profile_id_with_admin() {
    let app = build_app();
    let profile_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/social-links/profile/{}", profile_id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_social_link_with_admin() {
    let app = build_app();
    let body = json!({
        "profile_id": Uuid::new_v4(),
        "platform": "github",
        "url": "https://github.com/johndoe"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/social-links")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_social_link_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "url": "https://github.com/janedoe" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/social-links/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_social_link_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/social-links/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
}

// === Validation ===

#[tokio::test]
async fn test_create_social_link_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/social-links")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
