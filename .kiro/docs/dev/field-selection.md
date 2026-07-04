# QueryParams & Field Selection

Provides query parameter parsing for `includes` (eager loading with field selection) and `fields` (top-level field selection) across all API endpoints.

**Location:** `libs/shared/shared/data/core/src/query_params.rs`
**Field Filter Utility:** `libs/shared/shared/data/core/src/field_filter.rs`

---

## QueryParams Struct

```rust
#[derive(Deserialize, Default, Debug)]
pub struct QueryParams {
    includes: Vec<IncludeParam>,  // Parsed from bracket-aware split
    fields: Vec<String>,          // Comma-separated field names
}
```

Deserialized from query string via `Query<QueryParams>` in Axum handlers.

---

## IncludeParam

```rust
pub struct IncludeParam {
    pub name: String,                  // Relation name (e.g., "client")
    pub fields: Option<Vec<String>>,   // Optional field selection (e.g., ["id", "name"])
}
```

---

## Includes Syntax

The `includes` query parameter supports:

| Syntax | Meaning |
|--------|---------|
| `?includes=permissions` | Load all permission fields |
| `?includes=client` | Load all client fields |
| `?includes=permissions,client` | Load both relations |
| `?includes=client[id,name]` | Load client with only id and name |
| `?includes=permissions[id,resource],client[id,name]` | Field selection on both |

### Bracket-Aware Parsing

The parser splits on commas NOT inside `[]` brackets. So `permissions[id,resource],client[id,name]` correctly parses into two `IncludeParam` entries:
1. `{ name: "permissions", fields: Some(["id", "resource"]) }`
2. `{ name: "client", fields: Some(["id", "name"]) }`

---

## Fields Syntax

The `fields` query parameter selects top-level entity fields:

| Syntax | Meaning |
|--------|---------|
| `?fields=id,name` | Only return id and name |
| (no fields param) | Return all fields |

When `fields` is specified, included relations are **always preserved** regardless of whether they appear in `fields`. This means `?fields=id,name&includes=client[name]` returns id, name, and client.

---

## QueryParams Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `includes()` | `Vec<String>` | Relation names only (backward compat with Query macro) |
| `include_params()` | `&Vec<IncludeParam>` | Full include params with field selections |
| `include_fields(name)` | `Option<&Vec<String>>` | Field selection for a specific relation |
| `fields()` | `&Vec<String>` | Top-level field selection (empty = no filtering) |
| `add_includes(extra)` | `()` | Programmatically add includes (e.g., from related filters) |

---

## Field Selection Implementation Pattern

### In the Model Layer

Each response DTO implements a `filter_fields` method and uses `#[serde(skip_serializing_if = "Option::is_none")]`:

```rust
#[derive(Serialize, Debug, ToSchema, Default)]
pub struct ClientData {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    // ...
}

impl ClientData {
    pub fn filter_fields(mut self, fields: &Vec<String>) -> Self {
        if !fields.contains(&"id".to_string()) { self.id = None; }
        if !fields.contains(&"name".to_string()) { self.name = None; }
        // ...
        self
    }
}
```

### In the Parent DTO

The parent DTO (e.g., `RoleData`) has an `apply_field_filter` method:

```rust
impl RoleData {
    pub fn apply_field_filter(&mut self, query_params: &QueryParams) {
        let fields = query_params.fields();
        let includes = query_params.includes();

        // Filter top-level fields (skip relations that are in includes)
        if !fields.is_empty() {
            if !fields.contains(&"id".to_string()) { self.id = None; }
            // ... other fields ...
            if !fields.contains(&"client".to_string()) && !includes.contains(&"client".to_string()) {
                self.client = None;
            }
        }

        // Filter fields within included relations
        if let Some(selected) = query_params.include_fields("client") {
            self.client = self.client.take().map(|c| c.filter_fields(selected));
        }
        if let Some(selected) = query_params.include_fields("permissions") {
            self.permissions = self.permissions.take().map(|perms| {
                perms.into_iter().map(|p| p.filter_fields(selected)).collect()
            });
        }
    }
}
```

### In the Repo Layer

Called after entity-to-DTO conversion:

```rust
let mut role_data: RoleData = model.into();
role_data.apply_field_filter(query_params);
```

### API Routes

No changes needed — routes keep returning `ResponseJson<RoleData>`. Field filtering is applied in the repo layer, and `skip_serializing_if` ensures omitted fields don't appear in JSON.

---

## Generic Field Filter Utility (Alternative)

`libs/shared/shared/data/core/src/field_filter.rs` provides a generic alternative that works with any `T: Serialize` without per-struct boilerplate:

```rust
use shared_shared_data_core::field_filter::{apply_query_fields, apply_query_fields_to_query_result};

// Single item → serde_json::Value
let filtered = apply_query_fields(role_data, &query_params);

// QueryResult<T> → QueryResult<serde_json::Value>
let filtered = apply_query_fields_to_query_result(result, &query_params);
```

This approach:
- Serializes the struct to `serde_json::Value`
- Filters top-level keys (preserving included relations)
- Filters keys within included relation objects/arrays
- Removes null values

**Trade-off:** Returns `serde_json::Value` instead of the typed struct, so Swagger documentation may show `Value` instead of the proper type schema. Use the per-struct `apply_field_filter` approach (described above) when Swagger correctness is required.

---

## Example API Calls

```http
### Get roles with only id and name
GET /roles?fields=id,name&page=1&page_size=10

### Get roles with client name only
GET /roles?includes=client[name]&page=1&page_size=10

### Combine field selection with includes
GET /roles?fields=id,name&includes=permissions[id,resource],client[id,name]

### Get single role with all data
GET /roles/{role_id}?includes=permissions,client

### Get single role with filtered includes
GET /roles/{role_id}?includes=client[name]&fields=id,name
```
