use serde::Serialize;
use serde_json::{Map, Value};

use crate::paging::QueryResult;
use crate::query_params::QueryParams;

/// Apply field selection to any serializable struct based on query params.
///
/// - `?fields=id,name` → only keep top-level keys `id` and `name`
/// - `?includes=client[id,name]` → filter keys within the `client` relation
/// - Relations present in `includes` are always preserved regardless of `fields`
///
/// # Example
/// ```ignore
/// let role_data: RoleData = model.into();
/// let filtered = apply_query_fields(role_data, &query_params);
/// // `filtered` is a serde_json::Value with only the requested fields
/// ```
pub fn apply_query_fields<T: Serialize>(data: T, query_params: &QueryParams) -> Value {
    let mut value = serde_json::to_value(data).unwrap_or(Value::Null);

    if let Value::Object(ref mut map) = value {
        let fields = query_params.fields();
        let includes = query_params.includes();

        // Step 1: Filter top-level fields (preserving included relations)
        if !fields.is_empty() {
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                if !fields.contains(&key) && !includes.contains(&key) {
                    map.remove(&key);
                }
            }
        }

        // Step 2: Filter fields within included relations
        for include in query_params.include_params() {
            if let Some(ref selected_fields) = include.fields {
                if let Some(val) = map.get_mut(&include.name) {
                    filter_value_fields(val, selected_fields);
                }
            }
        }

        // Step 3: Remove null values for cleaner output
        remove_nulls(map);
    }

    value
}

/// Apply field selection to a list of serializable structs.
///
/// # Example
/// ```ignore
/// let roles: Vec<RoleData> = result.into_iter().map(|m| m.into()).collect();
/// let filtered = apply_query_fields_vec(roles, &query_params);
/// // `filtered` is a Vec<serde_json::Value>
/// ```
pub fn apply_query_fields_vec<T: Serialize>(data: Vec<T>, query_params: &QueryParams) -> Vec<Value> {
    data.into_iter()
        .map(|item| apply_query_fields(item, query_params))
        .collect()
}

/// Apply field selection to a QueryResult, returning QueryResult<serde_json::Value>.
///
/// # Example
/// ```ignore
/// let result: QueryResult<RoleData> = RoleQuery::search(...).await?;
/// let filtered = apply_query_fields_to_query_result(result, &query_params);
/// ```
pub fn apply_query_fields_to_query_result<T: Serialize>(
    result: QueryResult<T>,
    query_params: &QueryParams,
) -> QueryResult<Value> {
    QueryResult {
        total_page: result.total_page,
        result: apply_query_fields_vec(result.result, query_params),
    }
}

/// Filter keys within a JSON value (object or array of objects).
fn filter_value_fields(value: &mut Value, fields: &Vec<String>) {
    match value {
        Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                if !fields.contains(&key) {
                    map.remove(&key);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                if let Value::Object(map) = item {
                    let keys: Vec<String> = map.keys().cloned().collect();
                    for key in keys {
                        if !fields.contains(&key) {
                            map.remove(&key);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Remove null values from a JSON object for cleaner output.
fn remove_nulls(map: &mut Map<String, Value>) {
    let null_keys: Vec<String> = map
        .iter()
        .filter(|(_, v)| v.is_null())
        .map(|(k, _)| k.clone())
        .collect();
    for key in null_keys {
        map.remove(&key);
    }
}
