use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::Response,
    middleware::Next,
};
use serde_json::Value;
use std::collections::HashMap;

/// Middleware that filters JSON response fields based on `fields` query param.
/// Example: `?fields=id,code,name,items[code],items[name],items[meta]`
pub async fn field_filter_middleware(req: Request, next: Next) -> Response<Body> {
    // Only apply to GET requests
    if req.method() != http::Method::GET {
        return next.run(req).await;
    }

    let fields = parse_fields_from_query(req.uri().query());
    if fields.is_empty() {
        return next.run(req).await;
    }

    let selection = build_selection(&fields);
    let response = next.run(req).await;

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

    let filtered = filter_response(&selection, value);
    let filtered_bytes = serde_json::to_vec(&filtered).unwrap_or_default();
    Response::from_parts(parts, Body::from(filtered_bytes))
}

struct FieldSelection {
    top_level: Vec<String>,
    nested: HashMap<String, Vec<String>>,
}

fn parse_fields_from_query(query: Option<&str>) -> Vec<String> {
    let query = match query {
        Some(q) => q,
        None => return vec![],
    };
    for pair in query.split('&') {
        let mut kv = pair.splitn(2, '=');
        if let (Some("fields"), Some(val)) = (kv.next(), kv.next()) {
            if !val.is_empty() {
                return val.split(',').map(|s| s.trim().to_string()).collect();
            }
        }
    }
    vec![]
}

fn build_selection(fields: &[String]) -> FieldSelection {
    let mut top_level = Vec::new();
    let mut nested: HashMap<String, Vec<String>> = HashMap::new();
    for field in fields {
        if let Some(bracket_start) = field.find('[') {
            let parent = &field[..bracket_start];
            let inner = field[bracket_start + 1..].trim_end_matches(']');
            nested.entry(parent.to_string()).or_default().push(inner.to_string());
        } else {
            top_level.push(field.clone());
        }
    }
    FieldSelection { top_level, nested }
}

fn filter_response(selection: &FieldSelection, value: Value) -> Value {
    match &value {
        Value::Object(map) if map.contains_key("result") && map.contains_key("total_page") => {
            let mut out = serde_json::Map::new();
            out.insert("total_page".to_string(), map["total_page"].clone());
            if let Some(Value::Array(arr)) = map.get("result") {
                let filtered: Vec<Value> = arr.iter().map(|v| filter_object(selection, v.clone())).collect();
                out.insert("result".to_string(), Value::Array(filtered));
            }
            Value::Object(out)
        }
        _ => filter_object(selection, value),
    }
}

fn filter_object(selection: &FieldSelection, value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for (key, val) in map {
                if selection.top_level.contains(&key) {
                    filtered.insert(key, val);
                } else if let Some(nested_fields) = selection.nested.get(&key) {
                    filtered.insert(key, filter_nested(nested_fields, val));
                }
            }
            Value::Object(filtered)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|v| filter_object(selection, v)).collect())
        }
        other => other,
    }
}

fn filter_nested(fields: &[String], value: Value) -> Value {
    match value {
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|item| filter_nested(fields, item)).collect())
        }
        Value::Object(map) => {
            let mut filtered = serde_json::Map::new();
            for (key, val) in map {
                if fields.contains(&key) {
                    filtered.insert(key, val);
                }
            }
            Value::Object(filtered)
        }
        other => other,
    }
}
