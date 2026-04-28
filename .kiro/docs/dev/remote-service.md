# RemoteService Pattern

Inter-service communication using the `RemoteService` derive macro with Consul-based service discovery.

## Core Components

### Location
- Macro implementation: `libs/shared/shared/macro/src/service.rs`
- Macro registration: `libs/shared/shared/macro/src/lib.rs` (`#[derive(RemoteService)]`)
- QueryResult helper: `libs/shared/shared/data/core/src/paging.rs`
- Example services: `features/auth/remote/src/`, `features/email-template/remote/src/`

## Defining a Remote Service

```rust
use shared_shared_macro::RemoteService;

#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]  // Consul service name
pub struct PermissionService {}
```

The macro generates:
- `service_name()` — returns the Consul service name
- `http_protocol()` — reads `HTTP_PROTOCOL` env var (defaults to `"http"`)
- `update_remote(consul)` — discovers service instances from Consul, groups by tenant, stores in round-robin routing table
- `call_api(endpoint, method, json_body, headers)` — makes HTTP request to a discovered instance
- `get_instance(tenant_id)` — picks next instance via round-robin

## call_api Behavior

```rust
async fn call_api(
    endpoint: String,
    method: Method,              // GET, POST, PATCH
    json_body: Option<Value>,
    headers_hashmap: HashMap<String, String>,
) -> Result<Value, String>
```

- Extracts `tenant_id` from OpenTelemetry baggage context
- Picks a service instance for that tenant via round-robin
- Builds URL: `{protocol}://{ip}:{port}{endpoint}`
- Sends request with `Content-Type: application/json` + custom headers
- Parses response JSON and returns the `data` field: `response.data -> Value`
- Returns `Err(String)` on network errors, non-success status, parse failures, or missing `data` field

## Common Patterns

### GET with QueryResult (paginated list)

Use `QueryResult::from_value()` to deserialize paginated responses:

```rust
use shared_shared_data_core::paging::QueryResult;

let data = Self::call_api(url, reqwest::Method::GET, None, HashMap::new())
    .await
    .map_err(|e| e)?;
let result = QueryResult::<MyDataType>::from_value(data)?;
// result.total_page: u64
// result.result: Vec<MyDataType>
```

### GET with pagination loop (fetch all pages)

```rust
let mut page = 1u64;
let page_size = 20;
loop {
    let url = format!("{}?{}&page={}&page_size={}", endpoint, query_string, page, page_size);
    let res = Self::call_api(url, reqwest::Method::GET, None, headers.clone()).await;
    let data = match res {
        Ok(d) => d,
        Err(_) => break,
    };
    let total_page = data.get("total_page").and_then(|v| v.as_u64()).unwrap_or(0);
    let items = data.get("result"); // process items...

    if page >= total_page { break; }
    page += 1;
}
```

### POST with JSON body

```rust
use serde_json::json;

let body = json!({
    "email": email,
    "password": password,
});
let data = Self::call_api(endpoint, reqwest::Method::POST, Some(body), HashMap::new()).await?;
let result: MyType = serde_json::from_value(data).map_err(|e| e.to_string())?;
```

## Building Filter Queries

Use `FilterCondition` to build query string parameters:

```rust
use shared_shared_data_core::filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam};

// Single filter
let condition = FilterCondition::leaf(FilterEnum::String(FilterParam {
    name: "key".to_string(),
    operator: FilterOperator::Equal,
    value: Some(key.clone()),
    raw_value: key,
}));

// AND filter (multiple conditions)
let condition = FilterCondition::And(vec![
    FilterCondition::Leaf(FilterEnum::I32(FilterParam {
        name: "template_id".to_string(),
        operator: FilterOperator::Equal,
        value: Some(template_id),
        raw_value: template_id.to_string(),
    })),
    FilterCondition::Leaf(FilterEnum::String(FilterParam {
        name: "language_code".to_string(),
        operator: FilterOperator::Equal,
        value: Some(language.clone()),
        raw_value: language,
    })),
]);

let url = format!("{}?{}", endpoint, condition.to_query_string());
```

### Filter Operators
| Operator | Enum Variant | Query Format |
|----------|-------------|-------------|
| Equal | `FilterOperator::Equal` | `?key=eq\|value` |
| Like | `FilterOperator::Like` | `?key=li\|value` |
| StartWith | `FilterOperator::StartWith` | `?key=sw\|value` |
| In | `FilterOperator::In` | `?key=in\|v1,v2` |

## Adding a New Remote Service

1. Create crate: `features/{feature}/remote/`
2. Add `Cargo.toml` with dependencies: `shared-shared-macro`, `shared-shared-data-core`, `reqwest`, `serde_json`, model crate
3. Define the service struct:
```rust
#[derive(Debug, RemoteService)]
#[remote(name(my_service_name))]  // must match Consul service registration
pub struct MyRemoteService {}
```
4. Implement methods using `Self::call_api()`
5. Set endpoint env vars (e.g., `MY_ENDPOINT_SEARCH`)
6. Call `MyRemoteService::update_remote(&consul)` periodically to refresh service discovery

## Service Discovery Flow

```
Consul → update_remote() → TENANT_ROUTING_TABLE (global static)
                                ↓
Request → baggage.tenant_id → get_instance() → (ip, port) → call_api()
```

Each tenant gets its own round-robin pool of service instances.
