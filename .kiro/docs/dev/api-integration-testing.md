# API Integration Testing Patterns

Guide for writing integration tests for API services using MockDatabase, Axum Router, and tower::ServiceExt.

---

## 1. File Location

Integration tests live in `apis/<service>/tests/` as separate files (not inline `#[cfg(test)]` modules). Each file is compiled as its own crate.

```
apis/lookup/tests/
├── lookup_type_test.rs
├── lookup_item_test.rs
├── lookup_item_translation_test.rs
├── middleware_test.rs
└── field_filter_test.rs
```

---

## 2. Required Dependencies (dev-dependencies)

```toml
[dev-dependencies]
axum = { workspace = true }
chrono = { workspace = true }
http = { workspace = true }
http-body = { workspace = true }
sea-orm = { workspace = true, features = ["mock"] }
serde_json = { workspace = true }
tokio = { workspace = true }
tower = { workspace = true, features = ["util"] }
uuid = { workspace = true }
shared-shared-app = { workspace = true }
shared-shared-config = { workspace = true }
```

---

## 3. MockDatabase Setup

Use `std::sync::Once` to initialize the mock DB once across all tests in a file:

```rust
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::sync::{Arc, Once};
use shared_shared_config::db::{DB_READ, DB_WRITE};

static INIT: Once = Once::new();

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        // Add query results for SELECT operations (one per DB call)
        for _ in 0..40 {
            mock = mock.append_query_results(vec![vec![sample_model()]]);
        }
        // Add exec results for INSERT/UPDATE/DELETE operations
        mock = mock.append_exec_results(vec![
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
            MockExecResult { last_insert_id: 0, rows_affected: 1 },
        ]);
        let conn = Arc::new(mock.into_connection());
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}
```

**Important:** Paginated list queries use 2 DB calls (count + data), so provide enough mock results.

---

## 4. Building the Test App

```rust
use axum::{middleware, Router};
use shared_shared_app::{mapper::main_response_mapper, state::AppState};
use shared_shared_data_cache::cache::Cache;

fn build_app() -> Router {
    init_mock_db();

    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, LookupCacheState>::new("redis://127.0.0.1/", "test_prefix")
        .expect("Failed to create cache");
    let app_state = AppState::new(&db_conn, cache, Some(LookupAppState::default()));

    // Use the actual route function from the service
    api_lookup::routes::lookup_type::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}
```

---

## 5. Sending Requests with tower::ServiceExt

```rust
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_get_endpoint() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## 6. Parsing Response Body

```rust
use serde_json::Value;

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
```

The response wrapper format (from `main_response_mapper`):
```json
{
  "status": 1,
  "data": { /* actual response data */ }
}
```

---

## 7. Testing Auth/Permissions

Use the `baggage` header to simulate authenticated requests:

```rust
const BAGGAGE_ADMIN: &str = "accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000,tenant_id=test-tenant";

#[tokio::test]
async fn test_create_requires_auth() {
    let app = build_app();
    let body = json!({"code": "X", "name": "Y"});

    // Without baggage → rejected
    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_with_admin() {
    let app = build_app();
    let body = json!({"code": "X", "name": "Y"});

    // With baggage → allowed
    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header("content-type", "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

Public endpoints (using `PublicAccess` extractor) don't need the baggage header.

---

## 8. Testing Middleware in Isolation

Use mock handlers to test middleware behavior without real service logic:

```rust
use axum::{middleware, response::IntoResponse, routing::get, Json, Router};
use shared_shared_middleware::field_filter_middleware;

fn build_app() -> Router {
    async fn mock_handler() -> impl IntoResponse {
        Json(json!({
            "total_page": 1,
            "result": [{"id": "1", "code": "X", "name": "Y", "extra": "data"}]
        }))
    }

    Router::new()
        .route("/items", get(mock_handler))
        .layer(middleware::from_fn(field_filter_middleware))
}

#[tokio::test]
async fn test_middleware_filters_fields() {
    let app = build_app();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/items?fields=id,code")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let body = parse_body(response).await;
    assert!(body["result"][0].get("extra").is_none());
}
```

---

## 9. Testing Validation

```rust
#[tokio::test]
async fn test_validation_empty_field() {
    let app = build_app();
    let body = json!({"code": "", "name": "Valid"});

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header("content-type", "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_json() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types")
        .header("content-type", "application/json")
        .header("baggage", BAGGAGE_ADMIN)
        .body(Body::from("not valid json"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
```

---

## 10. Sample Model Factory

```rust
use chrono::Utc;
use uuid::Uuid;

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
```

---

## 11. Running Tests

```bash
# All tests for a service
cargo test -p api-lookup

# Specific test file
cargo test -p api-lookup --test lookup_type_test

# Specific test function
cargo test -p api-lookup --test lookup_type_test test_get_lookup_types_public_access

# With output
cargo test -p api-lookup -- --nocapture
```
