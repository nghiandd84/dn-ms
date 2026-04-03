# AGENTS.md — dn-ms Rust Monorepo

## Project Overview

Cargo workspace monorepo (Rust 1.85.0, resolver 2, edition 2021) with a layered microservices architecture. Uses Axum for HTTP, Sea-ORM for PostgreSQL, Redis, Kafka, OpenTelemetry, and Consul for service discovery.

## Build / Test Commands

```bash
# Build entire workspace
cargo build

# Build a specific crate
cargo build -p features-wallet-service
cargo build -p apis-wallet

# Run a specific binary
cargo run -p apis-wallet

# Run all tests
cargo test

# Run tests for a single crate
cargo test -p features-wallet-service

# Run a single test by name
cargo test -p shared-shared-data-core test_roundrobin_cycle

# Run tests matching a pattern
cargo test -p shared-shared-data-extractor idempotency

# Run async tests (requires tokio)
cargo test -p shared-shared-data-core -- --test-threads=1

# Check without building
cargo check
cargo check -p apis-auth

# Clippy (lint)
cargo clippy
cargo clippy -p features-booking-service

# Format
cargo fmt
cargo fmt -- --check

# Run migrations (example)
cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth
cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth down    # rollback
cargo run --bin migrations_auth -- -v -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s auth status  # check status
```

## Start Services

```bash
# Start infrastructure (PostgreSQL, Redis, Kafka, Consul)
bash start-service.sh

# Start all microservices (kills existing, rebuilds expected)
bash start.sh

# Kill all running services
bash start.sh kill

# Docker (infra without APIs)
docker-compose -f docker-compose.no_api.yml up -d
```

## Project Structure

```
apis/<name>/              — Top-level executable services (main.rs, routes, middleware)
features/<name>/          — Feature crates split into layers:
  entities/               —   Sea-ORM database models
  model/                  —   Request/Response DTOs, state structs
  repo/                   —   Data access layer (Query, Mutation)
  service/                —   Business logic
  migrations/             —   Sea-ORM migrations
  stream/                 —   Kafka consumers
  remote/                 —   Remote service clients
libs/shared/shared/       — Shared libraries:
  data/{app,auth,cache,core,error}  — Result types, errors, pagination, cache, auth
  app/                    — Generic AppState, config, startup
  observability/          — OpenTelemetry setup
  config/                 — Configuration loading
  extractor/              — Axum request extractors
  middleware/             — Axum middleware
  macro/                  — Procedural macros (Query, Mutation, Dto, Response, ParamFilter)
  macro-rule/             — Declarative macros (set_if_some!)
  auth/                   — Permission system
libs/tools/               — Tool libraries (dn-consul)
apps/                     — Standalone apps (gateway, notification, auth-web)
```

## Code Style

### Imports

Organize in four groups separated by blank lines:
1. External crates (alphabetical)
2. Shared libraries (`shared_shared_*`)
3. Feature-local crates (`features_*`)
4. Local crate imports (`crate::`)

```rust
use axum::{extract::Path, routing::get, Router};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use shared_shared_data_core::paging::{Pagination, QueryResult};
use shared_shared_data_error::app::AppError;

use features_wallet_model::wallet::{WalletData, WalletForCreateRequest};
use features_wallet_repo::wallet::{WalletMutation, WalletQuery};

use crate::wallet::util::assign;
```

### Naming Conventions

| Category | Convention | Examples |
|----------|-----------|----------|
| Structs | PascalCase | `WalletService`, `WalletForCreateRequest` |
| Enums | PascalCase | `AppError`, `ClientError` |
| Functions/Methods | snake_case | `create_wallet`, `filter_wallets` |
| Variables | snake_case | `wallet_id`, `pagination` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRIES`, `TAG` |
| Traits | PascalCase | `StartApp`, `IdempotencyCacheType` |
| Type aliases | PascalCase | `WalletQueryManager` |
| DTO suffixes | `*Request`, `*Data`, `*Dto` | `WalletForCreateRequest`, `WalletData` |
| Repo types | `*Query`, `*Mutation` | `WalletQuery`, `WalletMutation` |

### Error Handling

- Use `thiserror` for all error enums: `#[derive(Debug, Error)]`
- Core error type is `AppError` in `shared-shared-data-error`
- Use `#[from]` for automatic conversion (e.g., `sea_orm::DbErr`)
- Type alias: `pub type Result<T> = core::result::Result<T, AppError>;`
- Service layer wraps lower-level errors into `AppError::Internal("message")` with `debug!()` logging
- `AppError` implements `IntoResponse` for Axum — errors attach to response extensions
- Do NOT use `anyhow` — the codebase prefers explicit `thiserror`-based errors

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("Entity not found: {entity}")]
    EntityNotFound { entity: String },
    #[error("Internal error {0}")]
    Internal(String),
}
```

### Types & Patterns

- All service methods are `async fn` returning `Result<T, AppError>`
- Uses `tokio` runtime (`#[tokio::main]`)
- Some repo methods use `impl Future` pattern for trait-object compatibility
- `async_trait` from `async-trait` crate for async trait methods
- Unit-like structs for services: `pub struct WalletService {}`
- Newtype wrappers: `ResponseJson<T>(pub T)`, `ValidJson<T>(pub T)`
- `Uuid` (v4) for primary keys
- `chrono::NaiveDateTime` aliased as `DateTime` in models
- `Option<T>` for nullable/optional fields and partial updates
- `Vec<FilterEnum>` for dynamic filtering

### Formatting

- 4-space indentation
- Opening braces on same line as declaration
- Trailing commas in multi-line lists
- Max line length ~120 characters
- No `rustfmt.toml` exists — uses Rust defaults
- No `clippy.toml` exists

### Axum Patterns

- Routes defined as functions returning `Router` with `.route()` chaining
- `#[utoipa::path(...)]` attributes for OpenAPI docs on every handler
- `#[instrument(level = Level::INFO, skip_all)]` for tracing
- Custom extractors: `ValidJson<T>`, `ResponseJson<T>`
- State passed via `.with_state(app_state.clone())`
- `const TAG: &str = "feature_name"` for OpenAPI tag grouping

### Custom Procedural Macros

- `#[derive(Query)]` + `#[query(key_type(Uuid))]` — generates query methods
- `#[derive(Mutation)]` + `#[mutation(key_type(Uuid))]` — generates CRUD methods
- `#[derive(Dto)]` + `#[dto(name(...), columns(...), option)]` — generates DTOs
- `#[derive(Response)]` — generates response wrapper implementations
- `#[derive(ParamFilter)]` — generates filter parameter structs
- `set_if_some!` — declarative macro for conditional field assignment

### Tests

- Inline `mod tests` at bottom of source files: `#[cfg(test)] mod tests { use super::*; ... }`
- Sync tests: `#[test] fn test_name() { ... }`
- Async tests: `#[tokio::test] async fn test_name() { ... }`
- Panic tests: `#[test] #[should_panic(expected = "...")]`
- No integration test directories — tests are co-located with source
- Test doubles via trait impls: `impl TraitName for () { ... }`

### Logging

- Use `tracing` crate: `debug!()`, `warn!()`, `error!()`, `info!()`
- OpenTelemetry integration via `opentelemetry-otlp`
- `#[instrument(level = Level::INFO, skip_all)]` on handlers and service methods
