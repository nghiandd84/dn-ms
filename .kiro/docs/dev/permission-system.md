# Permission System (RBAC)

Role-based access control using Axum extractors with compile-time permission types.

## Core Components

### Location
- Trait & extractor: `libs/shared/shared/auth/src/permission.rs`
- Trait definition: `libs/shared/shared/auth/src/lib.rs`
- Permission map storage: `libs/shared/shared/app/src/state.rs` (`AppState.permissions_map`)
- Permission sync: `features/auth/remote/src/permission.rs` (`PermissionService::get_roles_by_service_name`)
- Baggage middleware: `require_baggage_header` in `permission.rs`

### Permission Constants
```rust
pub const READ: u32 = 1 << 0;   // 1
pub const CREATE: u32 = 1 << 1; // 2
pub const UPDATE: u32 = 1 << 2; // 4
pub const DELETE: u32 = 1 << 3; // 8
pub const ADMIN: u32 = 1 << 4;  // 16
```

### Super Admin Bypass
The role name `ADMIN_ALL` bypasses all permission checks. When the baggage header contains `accesses=ADMIN_ALL*,...`, the `Auth<R>` extractor short-circuits with `mask = u32::MAX` (all bits set) without any permission map lookup.

```rust
const SUPER_ADMIN_ROLE: &str = "ADMIN_ALL";
```

This is useful for:
- Testing via `test.rest` files
- Internal service-to-service calls
- Admin tooling

### ResourcePermission Trait
```rust
pub trait ResourcePermission {
    const BIT: u32;
    const RESOURCE: &'static str;
    fn requirements() -> &'static [(&'static str, u32)] { &[] }
}
```

## Defining Permissions

### Single Permission (most common)
Use `define_resource_perms!` macro in each API's `permission.rs`:

```rust
use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

const LOOKUP_TYPE_RESOURCE: &str = "LOOKUP_TYPE";

define_resource_perms! {
    CanCreateLookupType => (CREATE, LOOKUP_TYPE_RESOURCE),
    CanReadLookupType   => (READ, LOOKUP_TYPE_RESOURCE),
    CanUpdateLookupType => (UPDATE, LOOKUP_TYPE_RESOURCE),
    CanDeleteLookupType => (DELETE, LOOKUP_TYPE_RESOURCE)
}
```

### Multi-Permission (combined check)
Use `combine_perms!` when an endpoint requires multiple permissions:

```rust
use shared_shared_auth::{combine_perms, ResourcePermission};

combine_perms!(CanReadAndDeleteItem => [CanReadLookupItem, CanDeleteLookupItem]);
```

## Using in Route Handlers

Every handler must declare its access level using either `Auth<R>` or `PublicAccess` as the **first** parameter.

### Protected Endpoints
```rust
use shared_shared_auth::permission::Auth;
use crate::permission::{CanCreateLookupType, CanDeleteLookupType};

async fn create_lookup_type(
    _auth: Auth<CanCreateLookupType>,
    ValidJson(req): ValidJson<LookupTypeForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> { ... }

async fn delete_lookup_type(
    _auth: Auth<CanDeleteLookupType>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> { ... }
```

### Public Endpoints
```rust
use shared_shared_auth::permission::PublicAccess;

async fn get_lookup_types(
    _public: PublicAccess,
    query_pagination: Query<Pagination>,
) -> Result<ResponseJson<QueryResult<LookupTypeData>>> { ... }
```

### Convention
| HTTP Method | Extractor | Example |
|---|---|---|
| POST | `Auth<CanCreate*>` | `_auth: Auth<CanCreateEvent>` |
| GET | `Auth<CanRead*>` or `PublicAccess` | `_auth: Auth<CanReadEvent>` |
| PATCH | `Auth<CanUpdate*>` | `_auth: Auth<CanUpdateEvent>` |
| DELETE | `Auth<CanDelete*>` | `_auth: Auth<CanDeleteEvent>` |

Use `PublicAccess` for GET endpoints that should be accessible without authentication (e.g., lookup data, public listings).

### Accessing Auth Data
```rust
async fn handler(auth: Auth<CanUpdateItem>) -> Result<...> {
    let user_id = auth.user_id();       // Option<Uuid>
    let access_key = auth.access_key(); // Option<String>
    let mask = auth.mask;               // u32
}
```

## Baggage Header & Middleware

### Baggage Header Format
```
baggage: accesses=ROLE_A*key1|ROLE_B*,user_id=<uuid>,client_id=<uuid>
```

- `accesses`: pipe-separated list of `ROLE_NAME*ACCESS_KEY` pairs (key is optional)
- `user_id`: UUID of the authenticated user
- `client_id`: UUID of the client application

### require_baggage_header Middleware
Applied globally in `start_app.rs`. Rejects requests without a `baggage` header, except:
- Infrastructure paths: `/healthchecker`, `/swagger-ui`, `/api-docs`
- Paths listed in `public_paths()` override

```rust
fn public_paths(&self) -> &'static [&'static str] {
    &["/login", "/register", "/token"]
}
```

### How Auth Check Works
1. Middleware checks for `baggage` header presence (rejects if missing on non-public paths)
2. `Auth<R>` extractor parses baggage into `AccessTokenStruct`
3. If role is `ADMIN_ALL` → bypass, return `mask = u32::MAX`
4. Otherwise, for each user access (role), calls `state.get_permission_map(role_name, resource_name)`
5. Checks if the role's mask has the ADMIN bit OR the required permission bit
6. Returns the first matching role's mask and access key
7. If no role matches → returns `AuthError::InsufficientPermission`

## Permission Map Sync

Each service periodically fetches role→permission mappings from the auth service via Consul discovery:

```rust
fn custom_handler(&self, app_state: &mut AppState<T, C>) -> impl Future<Output = Result<...>> {
    let mut clone_app_state = app_state.clone();
    async move {
        spawn(async move {
            let service_key = "LOOKUP".to_string();
            let mut interval = interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let consul_client = get_consul_client().unwrap();
                PermissionService::update_remote(&consul_client).await;
                let all_permissions =
                    PermissionService::get_roles_by_service_name(service_key.clone()).await;
                for (role_name, permissions) in all_permissions {
                    let mask_permissions = permissions.iter()
                        .map(|p| (p.resource.clone().unwrap_or_default(), p.mask.unwrap_or(0) as u32))
                        .collect();
                    clone_app_state.set_permission_map(role_name, mask_permissions);
                }
            }
        });
        Ok(())
    }
}
```

The sync calls `GET /roles?includes=permissions&permissions[resource]=sw|SERVICE_KEY` on the auth service, paginates through all results, and updates the in-memory permission map.

## Adding Permissions to a New Service

### 1. Create permission.rs
```
apis/{service}/src/permission.rs
```
Define resource constants and permission structs using `define_resource_perms!`.

### 2. Register module
Add `mod permission;` to `apis/{service}/src/main.rs`.

### 3. Add dependencies (Cargo.toml)
```toml
shared-shared-auth = { workspace = true }
features-auth-remote = { workspace = true }
```

### 4. Add Auth extractors to route handlers
- POST → `_auth: Auth<CanCreate*>`
- GET → `_auth: Auth<CanRead*>` or `_public: PublicAccess`
- PATCH → `_auth: Auth<CanUpdate*>`
- DELETE → `_auth: Auth<CanDelete*>`

### 5. Add permission sync in app.rs
Add `custom_handler` with `PermissionService` sync loop (see above). If the service already has a `custom_handler` (e.g., for Kafka), add the permission sync spawn alongside it.

### 6. Override public_paths (if needed)
```rust
fn public_paths(&self) -> &'static [&'static str] {
    &["/some-public-endpoint"]
}
```

### 7. Update test.rest
Add baggage variable and header to test requests:
```
@baggage = accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000

### Create Resource
POST {{baseUrl}}/resources
Content-Type: application/json
baggage: {{baggage}}
```

## File Structure per API
```
apis/{service}/
├── Cargo.toml       # shared-shared-auth + features-auth-remote deps
├── test.rest        # @baggage variable, baggage header on protected requests
└── src/
    ├── main.rs      # mod permission;
    ├── permission.rs # define_resource_perms! for this service's resources
    ├── app.rs       # custom_handler with PermissionService sync + public_paths()
    └── routes/
        └── *.rs     # Auth<CanXxx> or PublicAccess as first handler param
```

## All Services with Permissions

| Service | Service Key | Permission File | Public Paths |
|---|---|---|---|
| auth | AUTH | 7 resources (user, role, permission, scope, client, token, auth_code) | `/login`, `/register`, `/token` |
| booking | BOOKING | 2 resources (booking, seat) | — |
| event | EVENT | 1 resource (event) | — |
| fee | FEE | 1 resource (configuration) | — |
| inventory | INVENTORY | 2 resources (seat, reservation) | — |
| merchant | MERCHANT | 3 resources (merchant, api_key, webhook) | — |
| profile | PROFILE | 3 resources (profile, preference, social_link) | — |
| translation | TRANSLATION | 4 resources (project, key, tag, version) | — |
| email-template | EMAIL_TEMPLATE | 3 resources (email_template, template_translation, template_placeholder) | — |
| wallet | WALLET | 6 resources (wallet, transaction, p2p_transfer, top_up, withdrawal, idempotency) | — |
| payment-core | PAYMENT_CORE | 4 resources (payment, method, attempt, method_limit) | — |
| stripe | PAYMENT_STRIPE | 4 resources (payment_intent, webhook_event, refund, api_log) | — |
| bakery | BAKERY | 7 resources (baker, bakery, cake, cake_baker, customer, order, lineitem) | — |
| lookup | LOOKUP | 3 resources (lookup_type, lookup_item, lookup_item_translation) | `/lookup-types` |
