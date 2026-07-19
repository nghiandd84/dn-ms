# Field-Level Access Control (FLAC)

Field-level permission system that controls which fields of a resource a role can READ or UPDATE. Built on top of the existing RBAC permission system.

## Design Decisions

| Aspect | Decision |
|--------|----------|
| Granularity | READ and UPDATE only |
| Default behavior | **Deny all** — no field permissions = no fields visible |
| Storage | Separate `field_permissions` table (role → resource → action → fields) |
| Response filtering | Omit unauthorized fields from response (partial JSON) |
| Scope | All services and entities |
| Multiple roles | UNION of all roles' allowed fields |
| `id` field | Always auto-included (never needs explicit listing) |
| Nested relations | Reference the nested entity's resource |
| Public APIs | Exempt — `PublicAccess` endpoints skip field filtering |
| Super Admin | `ADMIN_ALL` bypasses all field filtering |
| Migration location | Auth service |

---

## 1. Database Schema

New table in the auth service database:

```sql
CREATE TABLE field_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    resource VARCHAR(255) NOT NULL,
    action INTEGER NOT NULL,
    fields TEXT[] NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(role_id, resource, action)
);

CREATE INDEX idx_field_permissions_resource ON field_permissions(resource);
CREATE INDEX idx_field_permissions_role_id ON field_permissions(role_id);
```

### Column Definitions

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID | Primary key |
| `role_id` | UUID | FK to `roles.id` |
| `resource` | VARCHAR(255) | Resource identifier (e.g., `PROFILE:PROFILE`) — same format as existing permissions |
| `action` | INTEGER | Permission action bitmask: READ=1, UPDATE=4 |
| `fields` | TEXT[] | Postgres array of allowed field names |
| `created_at` | TIMESTAMP | Record creation time |
| `updated_at` | TIMESTAMP | Last update time |

### Constraints

- `UNIQUE(role_id, resource, action)` — one entry per role + resource + action combination
- `ON DELETE CASCADE` on `role_id` — removing a role removes its field permissions

---

## 2. Entity Model

```rust
// features/auth/entities/src/field_permission.rs

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "field_permissions")]
#[dto(name(FieldPermissionForCreate), columns(role_id, resource, action, fields))]
#[dto(name(FieldPermissionForUpdate), columns(fields), option)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub role_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub resource: String,
    pub action: i32,
    pub fields: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id"
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}
```

---

## 3. Auth Service API

New CRUD endpoints for managing field permissions:

```
POST   /field-permissions              — Create field permission
GET    /field-permissions              — List field permissions (filterable)
GET    /field-permissions/{id}         — Get single field permission
PATCH  /field-permissions/{id}         — Update field permission (fields array)
DELETE /field-permissions/{id}         — Delete field permission
```

### Query Support

```
GET /field-permissions?resource=sw|LOOKUP:&includes=role
GET /field-permissions?role_id=eq|<uuid>
```

### Response Model

```rust
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
pub struct FieldPermissionData {
    pub id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub role_name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<i32>,
    pub fields: Option<Vec<String>>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
```

---

## 4. In-Memory Storage

### New Field in AppState

```rust
// libs/shared/shared/app/src/state.rs

pub struct AppState<T, C = ()> {
    // ... existing fields ...
    pub permissions_map: Arc<Mutex<HashMap<String, Vec<(String, u32)>>>>,

    // NEW: field-level permissions
    pub field_permissions_map: Arc<Mutex<HashMap<String, Vec<FieldPermissionEntry>>>>,
}

#[derive(Clone, Debug)]
pub struct FieldPermissionEntry {
    pub resource: String,        // "LOOKUP:TYPE"
    pub action: u32,             // READ=1, UPDATE=4
    pub fields: Vec<String>,     // ["name", "code", "description"]
}
```

### StatePermission Trait Extension

```rust
pub trait StatePermission {
    fn get_permission_map(&self, role_name: String, resource_name: String) -> u32;
    fn pull_permission(&self) -> impl std::future::Future<Output = Result<(), AuthError>>;

    // NEW
    fn get_field_permissions(&self, role_name: &str, resource: &str, action: u32) -> Vec<String>;
}
```

Implementation:
```rust
fn get_field_permissions(&self, role_name: &str, resource: &str, action: u32) -> Vec<String> {
    let map = self.field_permissions_map.lock().unwrap_or_else(|p| p.into_inner());
    map.get(role_name)
        .map(|entries| {
            entries.iter()
                .filter(|e| e.resource == resource && e.action == action)
                .flat_map(|e| e.fields.clone())
                .collect()
        })
        .unwrap_or_default()
}
```

---

## 5. Permission Sync

### Remote Service Method

```rust
// features/auth/remote/src/permission.rs

impl PermissionService {
    /// Fetch field permissions for all roles that have entries for this service.
    pub async fn get_field_permissions_by_service_name(
        service_key: String,
    ) -> HashMap<String, Vec<FieldPermissionEntry>> {
        // GET /field-permissions?resource=sw|SERVICE_KEY:&includes=role
        // Paginate through results
        // Group by role_name → Vec<FieldPermissionEntry>
    }
}
```

### Enhanced custom_handler

```rust
// In each service's app.rs custom_handler:

spawn(async move {
    let service_key = "LOOKUP".to_string();
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        let consul_client = get_consul_client().unwrap();
        PermissionService::update_remote(&consul_client).await;

        // Existing: sync resource permissions
        let all_permissions = PermissionService::get_roles_by_service_name(service_key.clone()).await;
        for (role_name, permissions) in all_permissions {
            let mask_permissions = permissions.iter()
                .map(|p| (p.resource.clone().unwrap_or_default(), p.mask.unwrap_or(0) as u32))
                .collect();
            clone_app_state.set_permission_map(role_name, mask_permissions);
        }

        // NEW: sync field permissions
        let field_permissions = PermissionService::get_field_permissions_by_service_name(service_key.clone()).await;
        for (role_name, entries) in field_permissions {
            clone_app_state.set_field_permission_map(role_name, entries);
        }
    }
});
```

---

## 6. Auth Extractor Enhancement

### AllowedFields Extension

```rust
// libs/shared/shared/auth/src/permission.rs

/// Injected into request extensions by Auth<R> extractor.
/// Used by field_access_middleware to filter responses and validate updates.
#[derive(Clone, Debug)]
pub struct AllowedFields {
    /// None = admin/bypass (all fields allowed)
    /// Some(map) = resource → allowed field names
    pub read_fields: Option<HashMap<String, Vec<String>>>,
    pub update_fields: Option<HashMap<String, Vec<String>>>,
}
```

### Enhanced Auth<R>::from_request_parts

```rust
// After existing permission check passes:

if is_super_admin {
    parts.extensions.insert(AllowedFields {
        read_fields: None,
        update_fields: None,
    });
} else {
    let mut read_fields: HashMap<String, HashSet<String>> = HashMap::new();
    let mut update_fields: HashMap<String, HashSet<String>> = HashMap::new();

    for access in &access_token.accesses {
        // Get READ fields for this role + resource
        let rf = state.get_field_permissions(&access.role_name, R::RESOURCE, READ);
        read_fields.entry(R::RESOURCE.to_string()).or_default().extend(rf);

        // Get UPDATE fields for this role + resource
        let uf = state.get_field_permissions(&access.role_name, R::RESOURCE, UPDATE);
        update_fields.entry(R::RESOURCE.to_string()).or_default().extend(uf);
    }

    // Convert HashSet to Vec for storage
    parts.extensions.insert(AllowedFields {
        read_fields: Some(read_fields.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()),
        update_fields: Some(update_fields.into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect()),
    });
}
```

---

## 7. Response Filtering Middleware (READ)

```rust
// libs/shared/shared/middleware/src/field_access.rs

/// Middleware that filters JSON response fields based on the user's
/// field-level permissions. Only applies to GET requests.
///
/// - Reads `AllowedFields` from request extensions (set by Auth<R> extractor)
/// - If None (admin) or absent (public) → pass through
/// - Otherwise filters response JSON to only include allowed fields
/// - Always preserves the `id` field
/// - Handles QueryResult wrapper (total_page + result array)
/// - Handles nested relations via resource mapping
pub async fn field_access_middleware(req: Request, next: Next) -> Response<Body> {
    if req.method() != http::Method::GET {
        return next.run(req).await;
    }

    // Clone AllowedFields before passing request to next
    let allowed = req.extensions().get::<AllowedFields>().cloned();

    let response = next.run(req).await;

    match allowed {
        // No AllowedFields = PublicAccess endpoint, skip filtering
        None => response,
        // read_fields is None = ADMIN_ALL, skip filtering
        Some(ref af) if af.read_fields.is_none() => response,
        // Empty fields map = deny all (but id is always included)
        Some(af) => {
            let read_fields = af.read_fields.unwrap();
            filter_response_body(response, &read_fields).await
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

    let is_json = response.headers()
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
        Value::Object(map) if map.contains_key("result") && map.contains_key("total_page") => {
            let mut out = serde_json::Map::new();
            out.insert("total_page".to_string(), map["total_page"].clone());
            if let Some(Value::Array(arr)) = map.get("result") {
                let filtered: Vec<Value> = arr.iter()
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

fn filter_single_object(
    value: Value,
    allowed_fields: &HashMap<String, Vec<String>>,
) -> Value {
    // Get the primary resource fields (first entry in the map)
    // In practice, the resource is determined by the endpoint's Auth<R> extractor
    let primary_fields: Vec<&String> = allowed_fields.values().next()
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
                    if let Some(nested_fields) = find_nested_resource_fields(&key, allowed_fields) {
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
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|item| {
                filter_nested_object(item, allowed_fields)
            }).collect())
        }
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

/// Find field permissions for a nested relation key.
/// Each service must register its nested resource mappings.
fn find_nested_resource_fields(
    key: &str,
    allowed_fields: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    // Look through all resources in allowed_fields
    // The key corresponds to a nested resource if there's a matching resource entry
    // e.g., key="items" might map to "LOOKUP:ITEM" resource
    // This mapping is configured per-service
    allowed_fields.iter()
        .find(|(resource, _)| resource_matches_key(resource, key))
        .map(|(_, fields)| fields.clone())
}

fn resource_matches_key(resource: &str, key: &str) -> bool {
    // Convert resource like "LOOKUP:ITEM" to expected key "items" (pluralized lowercase entity)
    // This is a convention-based mapping
    let entity = resource.split(':').last().unwrap_or("");
    let expected_key = format!("{}s", entity.to_lowercase());
    expected_key == key || entity.to_lowercase() == key
}
```

---

## 8. UPDATE Validation Middleware

```rust
// libs/shared/shared/middleware/src/field_access.rs

/// Middleware that validates PATCH request bodies against field-level permissions.
/// Rejects requests that attempt to update fields the user doesn't have permission for.
///
/// Must run AFTER Auth<R> extractor has injected AllowedFields into extensions.
pub async fn field_update_guard(req: Request, next: Next) -> Response<Body> {
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
            let update_fields: Vec<&String> = af.update_fields.as_ref().unwrap()
                .values()
                .flat_map(|v| v.iter())
                .collect();

            // Read body to check field names
            let (parts, body) = req.into_parts();
            let bytes = match to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(_) => {
                    return (StatusCode::BAD_REQUEST, "Failed to read request body").into_response();
                }
            };

            if let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(&bytes) {
                for key in map.keys() {
                    if !update_fields.contains(&key) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(serde_json::json!({
                                "error": "FIELD_NOT_PERMITTED",
                                "message": format!("Field '{}' is not permitted for update", key),
                                "field": key
                            }))
                        ).into_response();
                    }
                }
            }

            // Reconstruct request with original body and continue
            let req = Request::from_parts(parts, Body::from(bytes));
            next.run(req).await
        }
    }
}
```

---

## 9. Middleware Layer Order

```rust
// In each service's routes() method:

fn routes(&self, app_state: &AppState<...>) -> Router {
    Router::new()
        .merge(entity_routes(app_state))
        .layer(middleware::from_fn(field_update_guard))       // Validate PATCH bodies
        .layer(middleware::from_fn(field_access_middleware))  // Filter GET responses
        .layer(middleware::from_fn(field_filter_middleware))  // User ?fields= filtering
}
```

**Execution order:**
- Request: `field_filter_middleware` → `field_access_middleware` → `field_update_guard` → handler
- Response: handler → `field_update_guard` → `field_access_middleware` → `field_filter_middleware`

The `field_update_guard` blocks unauthorized PATCH requests before reaching the handler.
The `field_access_middleware` filters GET response fields based on permissions.
The `field_filter_middleware` further filters by user's `?fields=` query param.

---

## 10. Nested Resource Mapping

Each service declares how JSON response keys map to resources. This is needed so the middleware knows which resource's field permissions to apply to nested arrays/objects.

```rust
// Configuration per service (e.g., in app.rs or a config module)

pub const NESTED_RESOURCE_MAP: &[(&str, &str)] = &[
    // (json_key, resource)
    ("items", "LOOKUP:ITEM"),
    ("translations", "LOOKUP:ITEM_TRANSLATION"),
];
```

Alternative: Use convention-based mapping where the JSON key is derived from the resource name:
- `LOOKUP:ITEM` → key `items` (lowercase entity + 's')
- `PROFILE:SOCIAL_LINK` → key `social_links` (lowercase + snake_case + 's')

---

## 11. Interaction with Existing Features

| Feature | Behavior |
|---------|----------|
| `?fields=id,name` | User-requested fields ∩ permission-allowed fields = final output |
| `?includes=items` | Nested relation filtered by its own resource's field permissions |
| `PublicAccess` endpoints | No `AllowedFields` in extensions → no filtering |
| `ADMIN_ALL` role | `AllowedFields { read_fields: None, update_fields: None }` → no filtering |
| No field_permission entry for role | Empty field set → response contains only `id` |
| Multiple roles | UNION of all roles' allowed fields |

---

## 12. Example Scenarios

### Scenario 1: READ with field restrictions

**Setup:**
```
Role: SUPPORT_AGENT
Resource: PROFILE:PROFILE
Action: READ (1)
Fields: ["name", "email", "created_at"]
```

**Request:** `GET /profiles/123`

**Response (filtered):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00"
}
```

Fields `phone`, `address`, `ssn`, `preferences` — all omitted.

### Scenario 2: UPDATE blocked field

**Setup:**
```
Role: SUPPORT_AGENT
Resource: PROFILE:PROFILE
Action: UPDATE (4)
Fields: ["name", "email"]
```

**Request:** `PATCH /profiles/123`
```json
{ "phone": "+1-555-0100" }
```

**Response:** `403 Forbidden`
```json
{
  "error": "FIELD_NOT_PERMITTED",
  "message": "Field 'phone' is not permitted for update",
  "field": "phone"
}
```

### Scenario 3: Multiple roles (UNION)

**Setup:**
```
Role: SUPPORT_AGENT → READ PROFILE:PROFILE → ["name", "email"]
Role: HR_STAFF      → READ PROFILE:PROFILE → ["name", "phone", "address"]
```

**User has both roles. Effective READ fields:**
```
["name", "email", "phone", "address"]  (union)
```

### Scenario 4: Nested relation filtering

**Setup:**
```
Role: VIEWER
Resource: LOOKUP:TYPE   → READ fields: ["code", "name"]
Resource: LOOKUP:ITEM   → READ fields: ["code", "value"]
```

**Request:** `GET /lookup-types/123?includes=items`

**Response:**
```json
{
  "id": "...",
  "code": "COUNTRY",
  "name": "Countries",
  "items": [
    { "id": "...", "code": "US", "value": "United States" },
    { "id": "...", "code": "UK", "value": "United Kingdom" }
  ]
}
```

### Scenario 5: No field permissions (deny-all)

**Setup:** Role `GUEST` has resource-level READ permission on `PROFILE:PROFILE` but NO entry in `field_permissions`.

**Request:** `GET /profiles/123`

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

Only `id` is returned (auto-included).

---

## 13. File Structure (Changes)

```
features/auth/
├── entities/src/
│   ├── field_permission.rs          # NEW: entity model
│   └── lib.rs                       # Add mod field_permission
├── model/src/
│   └── field_permission.rs          # NEW: FieldPermissionData, request types
├── repo/src/
│   └── field_permission.rs          # NEW: query/mutation
├── service/src/
│   └── field_permission.rs          # NEW: business logic
├── migrations/src/
│   └── mXXX_create_field_permissions.rs  # NEW: migration
└── remote/src/
    └── permission.rs                # MODIFIED: add get_field_permissions_by_service_name

apis/auth/src/
├── routes/
│   └── field_permission.rs          # NEW: CRUD endpoints
└── permission.rs                    # MODIFIED: add CanCreate/Read/Update/DeleteFieldPermission

libs/shared/shared/
├── app/src/
│   └── state.rs                     # MODIFIED: add field_permissions_map, FieldPermissionEntry
├── auth/src/
│   └── permission.rs                # MODIFIED: AllowedFields struct, enhanced Auth<R>
└── middleware/src/
    ├── field_access.rs              # NEW: field_access_middleware, field_update_guard
    └── lib.rs                       # MODIFIED: export new middleware

apis/*/src/
└── app.rs                           # MODIFIED: add field permission sync + middleware layers
```

---

## 14. Migration Path

1. Deploy auth service with new `field_permissions` table and CRUD API
2. Deploy shared libraries with `AllowedFields`, new middleware
3. Update each service's `app.rs` to sync field permissions and add middleware layers
4. Initially, configure all existing roles with full field lists (backward compatible)
5. Gradually restrict fields per role as needed

**Backward compatibility:** If `field_permissions_map` is empty for a role (no entries synced yet), the system can be configured to either:
- **Strict mode:** Deny all fields (only `id` returned) — matches deny-all default
- **Migration mode:** Allow all fields (skip filtering) — for gradual rollout

Recommended: Start in migration mode, switch to strict once all roles have field permissions configured.
