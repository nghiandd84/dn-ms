use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use std::collections::HashMap;

use shared_shared_auth::permission::AllowedFields;

/// Middleware that filters JSON response fields based on field-level permissions.
///
/// Only applies to GET requests with successful JSON responses.
/// Reads `AllowedFields` from request extensions (set by `Auth<R>` extractor).
///
/// Behavior:
/// - If `AllowedFields` is absent → pass through (PublicAccess endpoint)
/// - If `read_fields` is `None` → pass through (ADMIN_ALL bypass)
/// - Otherwise → filter response to only include allowed fields + always "id"
pub async fn field_access_middleware(req: Request, next: Next) -> Response<Body> {
    // Only filter GET responses
    if req.method() != http::Method::GET {
        return next.run(req).await;
    }

    // Extract AllowedFields before passing request to next
    let allowed = req.extensions().get::<AllowedFields>().cloned();

    let response = next.run(req).await;

    match allowed {
        // No AllowedFields = PublicAccess endpoint, skip filtering
        None => response,
        // read_fields is None = ADMIN_ALL, skip filtering
        Some(ref af) if af.read_fields.is_none() => response,
        // Apply field filtering
        Some(af) => {
            let read_fields = af.read_fields.unwrap();
            filter_response_body(response, &read_fields).await
        }
    }
}

/// Middleware that validates PATCH request bodies against field-level permissions.
/// Rejects requests that attempt to update fields the user doesn't have permission for.
///
/// Must run AFTER `Auth<R>` extractor has injected `AllowedFields` into extensions.
///
/// Behavior:
/// - Non-PATCH requests → pass through
/// - `AllowedFields` absent → pass through
/// - `update_fields` is `None` → pass through (ADMIN_ALL bypass)
/// - Otherwise → check all body keys are in allowed update fields
pub async fn field_update_guard(req: Request, next: Next) -> Response<Body> {
    // Only validate PATCH requests
    if req.method() != http::Method::PATCH {
        return next.run(req).await;
    }

    let allowed = req.extensions().get::<AllowedFields>().cloned();

    match allowed {
        // No AllowedFields = shouldn't happen on PATCH (requires Auth), pass through
        None => next.run(req).await,
        // update_fields is None = ADMIN_ALL, allow everything
        Some(ref af) if af.update_fields.is_none() => next.run(req).await,
        Some(af) => {
            let update_fields: Vec<String> = af
                .update_fields
                .as_ref()
                .unwrap()
                .values()
                .flat_map(|v| v.iter().cloned())
                .collect();

            // Read body to check field names
            let (parts, body) = req.into_parts();
            let bytes = match to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(_) => {
                    return (StatusCode::BAD_REQUEST, "Failed to read request body")
                        .into_response();
                }
            };

            if let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(&bytes) {
                for key in map.keys() {
                    if !update_fields.contains(key) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(serde_json::json!({
                                "error": "FIELD_NOT_PERMITTED",
                                "message": format!("Field '{}' is not permitted for update", key),
                                "field": key
                            })),
                        )
                            .into_response();
                    }
                }
            }

            // Reconstruct request with original body and continue
            let req = Request::from_parts(parts, Body::from(bytes));
            next.run(req).await
        }
    }
}

async fn filter_response_body(
    response: Response<Body>,
    allowed_fields: &HashMap<String, Vec<String>>,
) -> Response<Body> {
    if !response.status().is_success() {
        return response;
    }

    let is_json = response
        .headers()
        .get(http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/json"))
        .unwrap_or(false);

    if !is_json {
        return response;
    }

    let (parts, body) = response.into_parts();
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Response::from_parts(parts, Body::empty()),
    };

    let value: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => return Response::from_parts(parts, Body::from(bytes)),
    };

    let filtered = filter_value_by_permissions(value, allowed_fields);
    let filtered_bytes = serde_json::to_vec(&filtered).unwrap_or_default();
    Response::from_parts(parts, Body::from(filtered_bytes))
}

fn filter_value_by_permissions(
    value: Value,
    allowed_fields: &HashMap<String, Vec<String>>,
) -> Value {
    match &value {
        // QueryResult wrapper: { "total_page": N, "result": [...] }
        Value::Object(map)
            if map.contains_key("result") && map.contains_key("total_page") =>
        {
            let mut out = serde_json::Map::new();
            out.insert("total_page".to_string(), map["total_page"].clone());
            if let Some(Value::Array(arr)) = map.get("result") {
                let filtered: Vec<Value> = arr
                    .iter()
                    .map(|v| filter_single_object(v.clone(), allowed_fields))
                    .collect();
                out.insert("result".to_string(), Value::Array(filtered));
            }
            Value::Object(out)
        }
        // Single object
        _ => filter_single_object(value, allowed_fields),
    }
}

fn filter_single_object(value: Value, allowed_fields: &HashMap<String, Vec<String>>) -> Value {
    // Get the primary resource fields (first entry in the map)
    let primary_fields: Vec<&String> = allowed_fields
        .values()
        .next()
        .map(|v| v.iter().collect())
        .unwrap_or_default();

    match value {
        Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for (key, val) in map {
                // "id" always included
                if key == "id" {
                    filtered.insert(key, val);
                    continue;
                }
                // Check if this is a nested relation with its own resource permissions
                if val.is_array() || val.is_object() {
                    if let Some(nested_fields) =
                        find_nested_resource_fields(&key, allowed_fields)
                    {
                        filtered.insert(key, filter_nested_value(val, &nested_fields));
                        continue;
                    }
                }
                // Top-level field check
                if primary_fields.contains(&&key) {
                    filtered.insert(key, val);
                }
            }
            Value::Object(filtered)
        }
        other => other,
    }
}

fn filter_nested_value(value: Value, allowed_fields: &[String]) -> Value {
    match value {
        Value::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|item| filter_nested_object(item, allowed_fields))
                .collect(),
        ),
        Value::Object(_) => filter_nested_object(value, allowed_fields),
        other => other,
    }
}

fn filter_nested_object(value: Value, allowed_fields: &[String]) -> Value {
    match value {
        Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for (key, val) in map {
                if key == "id" || allowed_fields.contains(&key) {
                    filtered.insert(key, val);
                }
            }
            Value::Object(filtered)
        }
        other => other,
    }
}

/// Find field permissions for a nested relation key by matching resource names.
///
/// Convention: a JSON key like "items" matches a resource whose entity part
/// (after the colon) lowercased + "s" equals the key.
/// e.g., "LOOKUP:ITEM" → "items", "LOOKUP:ITEM_TRANSLATION" → "item_translations"
fn find_nested_resource_fields(
    key: &str,
    allowed_fields: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    allowed_fields
        .iter()
        .find(|(resource, _)| resource_matches_key(resource, key))
        .map(|(_, fields)| fields.clone())
}

fn resource_matches_key(resource: &str, key: &str) -> bool {
    let entity = match resource.split(':').last() {
        Some(e) => e,
        None => return false,
    };
    // Convert UPPER_SNAKE_CASE entity to lowercase_snake_case + "s" for plural
    let expected_key = format!("{}s", entity.to_lowercase());
    expected_key == key || entity.to_lowercase() == key
}
