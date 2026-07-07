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

use features_profiles_entities::profile::Model as ProfileModel;
use features_profiles_model::state::{ProfileAppState, ProfileCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> ProfileModel {
    ProfileModel {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        bio: "Software engineer".to_string(),
        avatar_url: "https://example.com/avatar.png".to_string(),
        location: "New York".to_string(),
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
    let cache = Cache::<String, ProfileCacheState>::new("redis://127.0.0.1/", "test_profile")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(ProfileAppState::default()));

    api_profile::routes::profile::routes(&app_state)
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
async fn test_create_profile_requires_auth() {
    let app = build_app();
    let body = json!({
        "user_id": Uuid::new_v4(),
        "first_name": "John",
        "last_name": "Doe"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/profiles")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_profile_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "first_name": "Jane" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/profiles/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_profile_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/profiles/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access ===

#[tokio::test]
async fn test_get_profiles_with_admin() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/profiles")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    assert_ne!(status, StatusCode::FORBIDDEN);
    assert_ne!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_profile_by_id_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/profiles/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_profile_by_user_id_with_admin() {
    let app = build_app();
    let user_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/profiles/user/{}", user_id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_profile_with_admin() {
    let app = build_app();
    let body = json!({
        "user_id": Uuid::new_v4(),
        "first_name": "John",
        "last_name": "Doe"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/profiles")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_profile_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "first_name": "Jane" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/profiles/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_profile_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/profiles/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
}

// === Validation ===

#[tokio::test]
async fn test_create_profile_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/profiles")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
