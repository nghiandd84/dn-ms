# DN Microservices — Project Guide

## Overview

Rust monorepo of microservices for event ticketing/booking/merchant management with payments, wallet, notifications, i18n, and a Pingora-based gateway.

## Architecture

```
apps/     → Standalone apps (gateway, notification, auth-notification)
apis/     → 17 HTTP API services (Axum routers, handlers, middleware)
features/ → Domain modules split into sub-layers:
  entities/   SeaORM database models
  model/      DTOs, request/response types, state structs
  repo/       Data access layer (queries with Query/Macro derive macros)
  service/    Business logic, orchestration, validation
  migrations/ Schema migrations
  stream/     Kafka consumers/event processing
  remote/     Inter-service HTTP clients
libs/     → Shared libraries (config, auth, observability, macros, middleware, extractors)
docker/   → Docker configs and compose files
```

### Layered Flow

```
Client → API Layer (Axum router/handler) → Service Layer (business logic)
  → Repo Layer (SeaORM queries) → Entities/Model Layer (DB mapping)
  → Remote Clients (inter-service HTTP) or Kafka Stream (events)
Middlewares & Extractors run at the API layer.
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Runtime | Rust 1.85+, Tokio |
| HTTP | Axum |
| ORM | SeaORM (PostgreSQL) |
| Cache | Redis |
| Messaging | Kafka (rdkafka, KRaft mode) |
| Service Discovery | Consul |
| Observability | OpenTelemetry, Tracing, Jaeger |
| Auth | JWT (jsonwebtoken), Argon2, RBAC |
| API Docs | Utoipa + Swagger UI |
| Gateway | Pingora (reverse proxy with interceptors) |

## Permission System (RBAC)

Each API service defines its own permissions in `permission.rs` using `define_resource_perms!`:

```rust
use shared_shared_auth::{ define_resource_perms, permission::{CREATE, DELETE, READ, UPDATE} };
const PROFILE_RESOURCE: &str = "PROFILE:PROFILE";
define_resource_perms! {
    CanCreateProfile => (CREATE, PROFILE_RESOURCE),
    CanReadProfile   => (READ, PROFILE_RESOURCE),
    CanUpdateProfile => (UPDATE, PROFILE_RESOURCE),
    CanDeleteProfile => (DELETE, PROFILE_RESOURCE)
}
```

Resource naming: `SERVICE_KEY:ENTITY_KEY` (colon separator, UPPER_SNAKE_CASE).

### Handler Extractors

| Method | Extractor | Example |
|--------|-----------|---------|
| POST | `Auth<CanCreate*>` | `_auth: Auth<CanCreateProfile>` |
| GET | `Auth<CanRead*>` or `PublicAccess` | `_auth: Auth<CanReadProfile>` |
| PATCH | `Auth<CanUpdate*>` | `_auth: Auth<CanUpdateProfile>` |
| DELETE | `Auth<CanDelete*>` | `_auth: Auth<CanDeleteProfile>` |

`PublicAccess` is for GET endpoints that need no authentication (e.g., lookup data).

### Super Admin Bypass

Role name `ADMIN_ALL` bypasses all permission checks. In baggage header:
```
baggage: accesses=ADMIN_ALL*,user_id=<uuid>,client_id=<uuid>
```

### Permission Sync

Each service periodically fetches role→permission mappings from auth service via a `custom_handler` loop (every 30s) using `PermissionService::get_roles_by_service_name`.

## Testing Patterns

### Unit/Integration Tests

- Mock DB with `sea_orm::MockDatabase` + `Once` guard for global init
- `tower::ServiceExt::oneshot` for HTTP request testing
- `baggage` header with `ADMIN_ALL*` for auth bypass
- `main_response_mapper` middleware applied via `.layer(middleware::map_response(main_response_mapper))`

### Creating Tests for a New API Service

1. Add `[[bin]]`, `[lib]`, and `[dev-dependencies]` to `Cargo.toml`
2. Create `src/lib.rs` exporting `pub mod routes; pub mod permission;`
3. Create `tests/` directory with test files

```rust
static INIT: Once = Once::new();

fn init_mock_db() {
    INIT.call_once(|| {
        let mut mock = MockDatabase::new(DatabaseBackend::Postgres);
        for _ in 0..40 { mock = mock.append_query_results(vec![vec![sample_model()]]); }
        mock = mock.append_exec_results(vec![
            MockExecResult { last_insert_id: 0, rows_affected: 1 }, /* repeat ~10x */
        ]);
        let conn = Arc::new(mock.into_connection());
        let _ = DB_READ.set(conn.clone());
        let _ = DB_WRITE.set(conn);
    });
}

fn build_app() -> Router {
    init_mock_db();
    let db_conn = DB_WRITE.get().unwrap().as_ref().clone();
    let cache = Cache::<String, MyCacheState>::new("redis://127.0.0.1/", "test").expect("cache");
    let app_state = AppState::new(&db_conn, cache, Some(MyAppState::default()));
    api_myservice::routes::myentity::routes(&app_state)
        .layer(middleware::map_response(main_response_mapper))
}
```

### API Integration Tests

Each service has a `test.rest` file (VS Code REST Client format) with sample requests using `@baggage = accesses=ADMIN_ALL*,user_id=...,client_id=...`.

## Query & Field Selection

### QueryParams

Parsed from `?includes=...&fields=...` via `Query<QueryParams>`.

| Syntax | Meaning |
|--------|---------|
| `?includes=permissions` | Load all permission fields |
| `?includes=client[id,name]` | Load client with only id and name |
| `?includes=permissions[id,resource],client[id,name]` | Field selection on both |
| `?fields=id,name` | Only return id and name at top level |

### FilterCondition

| Query | Meaning |
|-------|---------|
| `?name=eq\|admin` | name = "admin" |
| `?name=li\|admin` | name LIKE "%admin%" |
| `?name=sw\|LOOKUP` | name LIKE "LOOKUP%" |
| `?name=in\|A,B` | name IN ("A", "B") |
| `?name=eq\|admin&status=eq\|active&_condition=or` | name="admin" OR status="active" |

### Query Macro

`#[derive(Query)]` on a struct generates `QueryManager` implementation:
- `#[query(key_type(Uuid))]` — primary key type
- `#[query_filter(column_name(Column))]` — SeaORM column enum for filtering
- `#[query_related(entity(...), column(...), field(...), name("..."))]` — eager loading with optional cross-entity filtering

### Dto Macro

`#[derive(Dto)]` on SeaORM entity Model generates:
- `#[dto(name(MyForCreate), columns(field1, field2))]` — create DTO struct
- `#[dto(name(MyForUpdate), columns(field1, field2), option)]` — update DTO struct (all fields Option)

## Transaction Pattern

For multi-write operations, use SeaORM transactions:

```rust
use sea_orm::TransactionTrait;
use shared_shared_config::db::DB_WRITE;

let db = DB_WRITE.get().expect("DB_WRITE not initialized");
let txn = db.begin().await.map_err(|_| AppError::Unknown)?;
// All writes use &txn
SomeMutation::create_with_txn(dto, &txn).await?;
txn.commit().await.map_err(|_| AppError::Unknown)?;
// Side effects (Kafka, HTTP) AFTER commit
```

## Remote Services (Inter-service HTTP)

`#[derive(RemoteService)]` with `#[remote(name(consul_service_name))]` generates:
- `call_api(endpoint, method, json_body, headers)` — HTTP to discovered instance
- `update_remote(consul)` — refresh instances from Consul
- Round-robin routing per tenant

## Migrations

```bash
export DATABASE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
cargo run --bin migrations_<service> -- -v -u $DATABASE_URL -s <service>
```

Rollback: add `down` flag. Status: add `status` flag.

## Gateway Interceptors

Pingora-based reverse proxy with plugin interceptors:
- `request_id` — inject X-Request-Id
- `token_auth` — verify JWT, inject baggage header
- `cors` — CORS handling
- `rate_limiter` — token bucket per client IP

Configured in `apps/gateway/config/config.yaml`. Each interceptor is scoped via `filter` to specific routes.

## Conventional Commits

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

| Type | Purpose |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation |
| `refactor` | Code refactor |
| `test` | Add/update tests |
| `chore` | Maintenance |
| `perf` | Performance improvement |
| `build` | Build system/dependencies |

Breaking changes: `feat!: description` or `BREAKING CHANGE:` footer.
