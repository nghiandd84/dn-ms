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

use features_profiles_entities::user_preference::Model as UserPreferenceModel;
use features_profiles_model::state::{ProfileAppState, ProfileCacheState};

const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

static INIT: Once = Once::new();

fn sample_model() -> UserPreferenceModel {
    UserPreferenceModel {
        id: Uuid::new_v4(),
        profile_id: Uuid::new_v4(),
        language: "en".to_string(),
        theme: "light".to_string(),
        notifications_enabled: true,
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
    let cache = Cache::<String, ProfileCacheState>::new("redis://127.0.0.1/", "test_pref")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(ProfileAppState::default()));

    api_profile::routes::user_preference::routes(&app_state)
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
async fn test_create_preference_requires_auth() {
    let app = build_app();
    let body = json!({
        "profile_id": Uuid::new_v4(),
        "language": "en",
        "theme": "light"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/user-preferences")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_preference_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "theme": "dark" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/user-preferences/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_preference_requires_auth() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/user-preferences/{}", id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// === Admin access ===

#[tokio::test]
async fn test_get_preferences_with_admin() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/user-preferences")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_preference_by_id_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/user-preferences/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_preference_by_profile_id_with_admin() {
    let app = build_app();
    let profile_id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("/user-preferences/profile/{}", profile_id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_preference_with_admin() {
    let app = build_app();
    let body = json!({
        "profile_id": Uuid::new_v4(),
        "language": "en",
        "theme": "light"
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri("/user-preferences")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_preference_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();
    let body = json!({ "theme": "dark" });

    let req = Request::builder()
        .method(Method::PATCH)
        .uri(format!("/user-preferences/{}", id))
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_delete_preference_with_admin() {
    let app = build_app();
    let id = Uuid::new_v4();

    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/user-preferences/{}", id))
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::FORBIDDEN);
}

// === Validation ===

#[tokio::test]
async fn test_create_preference_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/user-preferences")
        .header(header::CONTENT_TYPE, "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
