use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    middleware,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use shared_shared_middleware::field_filter_middleware;

fn build_app() -> Router {
    async fn mock_list_handler() -> impl IntoResponse {
        Json(json!({
            "total_page": 1,
            "result": [
                {
                    "id": "00000000-0000-0000-0000-000000000001",
                    "tenant_id": "test-tenant",
                    "code": "CURRENCY",
                    "name": "Currency Types",
                    "description": "All currency types",
                    "is_active": true,
                    "created_at": "2024-01-01T00:00:00",
                    "updated_at": "2024-01-01T00:00:00",
                    "items": [
                        {
                            "id": "00000000-0000-0000-0000-000000000010",
                            "code": "USD",
                            "name": "US Dollar",
                            "meta": {"symbol": "$"},
                            "is_active": true,
                            "sort_order": 1
                        },
                        {
                            "id": "00000000-0000-0000-0000-000000000011",
                            "code": "EUR",
                            "name": "Euro",
                            "meta": {"symbol": "€"},
                            "is_active": true,
                            "sort_order": 2
                        }
                    ]
                }
            ]
        }))
    }

    async fn mock_single_handler() -> impl IntoResponse {
        Json(json!({
            "id": "00000000-0000-0000-0000-000000000001",
            "tenant_id": "test-tenant",
            "code": "CURRENCY",
            "name": "Currency Types",
            "description": "All currency types",
            "is_active": true,
            "created_at": "2024-01-01T00:00:00",
            "updated_at": "2024-01-01T00:00:00",
            "items": [
                {
                    "id": "00000000-0000-0000-0000-000000000010",
                    "code": "USD",
                    "name": "US Dollar",
                    "meta": {"symbol": "$"},
                    "is_active": true
                }
            ]
        }))
    }

    async fn mock_items_list_handler() -> impl IntoResponse {
        Json(json!({
            "total_page": 1,
            "result": [
                {
                    "id": "00000000-0000-0000-0000-000000000010",
                    "lookup_type_id": "00000000-0000-0000-0000-000000000001",
                    "code": "USD",
                    "name": "US Dollar",
                    "url": "",
                    "meta": {"symbol": "$"},
                    "is_active": true,
                    "sort_order": 1,
                    "created_at": "2024-01-01T00:00:00",
                    "updated_at": "2024-01-01T00:00:00"
                }
            ]
        }))
    }

    async fn mock_item_single_handler() -> impl IntoResponse {
        Json(json!({
            "id": "00000000-0000-0000-0000-000000000010",
            "lookup_type_id": "00000000-0000-0000-0000-000000000001",
            "code": "USD",
            "name": "US Dollar",
            "url": "",
            "meta": {"symbol": "$"},
            "is_active": true,
            "sort_order": 1
        }))
    }

    Router::new()
        .route("/lookup-types", get(mock_list_handler))
        .route("/lookup-types/{id}", get(mock_single_handler))
        .route("/lookup-types/{type_code}/items", get(mock_items_list_handler))
        .route("/lookup-types/{type_code}/items/{id}", get(mock_item_single_handler))
        .layer(middleware::from_fn(field_filter_middleware))
}

async fn parse_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// === GET /lookup-types tests ===

#[tokio::test]
async fn test_get_lookup_types_no_fields_returns_all() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert!(item["id"].is_string());
    assert!(item["tenant_id"].is_string());
    assert!(item["code"].is_string());
    assert!(item["name"].is_string());
    assert!(item["description"].is_string());
    assert!(item["is_active"].is_boolean());
    assert!(item["items"].is_array());
}

#[tokio::test]
async fn test_get_lookup_types_with_fields_filters_top_level() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types?fields=id,code,name")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert!(item["id"].is_string());
    assert!(item["code"].is_string());
    assert!(item["name"].is_string());
    assert!(item.get("tenant_id").is_none());
    assert!(item.get("description").is_none());
    assert!(item.get("is_active").is_none());
    assert!(item.get("items").is_none());
}

#[tokio::test]
async fn test_get_lookup_types_with_nested_fields() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types?fields=id,code,name,items[code],items[name],items[meta]")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert_eq!(item["id"], "00000000-0000-0000-0000-000000000001");
    assert_eq!(item["code"], "CURRENCY");
    assert_eq!(item["name"], "Currency Types");
    assert!(item.get("description").is_none());

    let items = item["items"].as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["code"], "USD");
    assert_eq!(items[0]["name"], "US Dollar");
    assert_eq!(items[0]["meta"]["symbol"], "$");
    assert!(items[0].get("id").is_none());
    assert!(items[0].get("is_active").is_none());
    assert!(items[0].get("sort_order").is_none());

    assert_eq!(items[1]["code"], "EUR");
    assert_eq!(items[1]["name"], "Euro");
    assert_eq!(items[1]["meta"]["symbol"], "€");
}

// === GET /lookup-types/{id} tests ===

#[tokio::test]
async fn test_get_lookup_type_by_id_with_fields() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/00000000-0000-0000-0000-000000000001?fields=id,code,items[code],items[meta]")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["id"], "00000000-0000-0000-0000-000000000001");
    assert_eq!(body["code"], "CURRENCY");
    assert!(body.get("name").is_none());
    assert!(body.get("tenant_id").is_none());
    assert!(body.get("description").is_none());

    let items = body["items"].as_array().unwrap();
    assert_eq!(items[0]["code"], "USD");
    assert_eq!(items[0]["meta"]["symbol"], "$");
    assert!(items[0].get("name").is_none());
    assert!(items[0].get("id").is_none());
}

// === GET /lookup-types/{type_code}/items tests ===

#[tokio::test]
async fn test_get_lookup_items_no_fields_returns_all() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/CURRENCY/items")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert!(item["id"].is_string());
    assert!(item["code"].is_string());
    assert!(item["name"].is_string());
    assert!(item["meta"].is_object());
    assert!(item["is_active"].is_boolean());
}

#[tokio::test]
async fn test_get_lookup_items_with_fields() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/CURRENCY/items?fields=code,name,meta")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert_eq!(item["code"], "USD");
    assert_eq!(item["name"], "US Dollar");
    assert_eq!(item["meta"]["symbol"], "$");
    assert!(item.get("id").is_none());
    assert!(item.get("lookup_type_id").is_none());
    assert!(item.get("is_active").is_none());
    assert!(item.get("sort_order").is_none());
}

// === GET /lookup-types/{type_code}/items/{id} tests ===

#[tokio::test]
async fn test_get_lookup_item_by_id_with_fields() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types/CURRENCY/items/00000000-0000-0000-0000-000000000010?fields=code,name,meta")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    assert_eq!(body["code"], "USD");
    assert_eq!(body["name"], "US Dollar");
    assert_eq!(body["meta"]["symbol"], "$");
    assert!(body.get("id").is_none());
    assert!(body.get("lookup_type_id").is_none());
    assert!(body.get("is_active").is_none());
}

// === Middleware behavior tests ===

#[tokio::test]
async fn test_field_filter_does_not_apply_to_post() {
    async fn mock_post_handler() -> impl IntoResponse {
        Json(json!({"ok": true, "id": "123", "extra": "data"}))
    }

    let app = Router::new()
        .route("/lookup-types", axum::routing::post(mock_post_handler))
        .layer(middleware::from_fn(field_filter_middleware));

    let req = Request::builder()
        .method(Method::POST)
        .uri("/lookup-types?fields=id")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"code":"X","name":"Y"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let body = parse_body(response).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["id"], "123");
    assert_eq!(body["extra"], "data");
}

#[tokio::test]
async fn test_field_filter_preserves_total_page() {
    let app = build_app();

    let req = Request::builder()
        .method(Method::GET)
        .uri("/lookup-types?fields=id")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let body = parse_body(response).await;
    assert_eq!(body["total_page"], 1);
    let item = &body["result"][0];
    assert!(item["id"].is_string());
    assert!(item.get("code").is_none());
}
