# SKILL.md — dn-ms Rust Monorepo

## Purpose

This SKILL.md documents key coding conventions, workflows, and reusable patterns for the dn-ms Rust monorepo. It is intended for both AI coding agents and human developers to ensure consistency, accelerate onboarding, and enable effective automation.

## Project Overview

- **Language:** Rust 1.85.0+, Edition 2021
- **Architecture:** Layered microservices (Axum, Sea-ORM, PostgreSQL, Redis, Kafka, OpenTelemetry, Consul)
- **Structure:**
  - `apis/<name>/` — Executable services (main.rs, routes, middleware)
  - `features/<name>/` — Feature crates (entities, model, repo, service, migrations, stream, remote)
  - `libs/shared/` — Shared libraries (data, app, observability, config, extractor, middleware, macro, auth)
  - `libs/tools/` — Tool libraries (e.g., dn-consul)
  - `apps/` — Standalone apps (gateway, notification, auth-web)

## Key Conventions

### Imports
- Four groups, separated by blank lines:
  1. External crates (alphabetical)
  2. Shared libraries (`shared_shared_*`)
  3. Feature-local crates (`features_*`)
  4. Local crate imports (`crate::`)

### Naming
- Structs/Enums/Traits/Type Aliases: PascalCase
- Functions/Methods/Variables: snake_case
- Constants: SCREAMING_SNAKE_CASE
- DTOs: `*Request`, `*Data`, `*Dto`
- Repo types: `*Query`, `*Mutation`

### Error Handling
- Use `thiserror` for error enums
- Core error: `AppError` in `shared-shared-data-error`
- Type alias: `pub type Result<T> = core::result::Result<T, AppError>;`
- No `anyhow` — use explicit error types

### Types & Patterns
- All service methods: `async fn` returning `Result<T, AppError>`
- Uses `tokio` runtime
- `async_trait` for async trait methods
- Unit-like structs for services
- Newtype wrappers: `ResponseJson<T>`, `ValidJson<T>`
- `Uuid` for primary keys
- `Option<T>` for nullable fields
- `Vec<FilterEnum>` for dynamic filtering

### Formatting
- 4-space indentation
- Opening braces on same line
- Trailing commas in multi-line lists
- Max line length ~120 characters
- Use Rust defaults (no rustfmt.toml)

### Axum Patterns
- Routes as functions returning `Router` with `.route()` chaining
- `#[utoipa::path(...)]` for OpenAPI docs
- `#[instrument(level = Level::INFO, skip_all)]` for tracing
- Custom extractors: `ValidJson<T>`, `ResponseJson<T>`
- State via `.with_state(app_state.clone())`
- `const TAG: &str = "feature_name"` for OpenAPI grouping

### Procedural Macros
- `#[derive(Query)]`, `#[derive(Mutation)]`, `#[derive(Dto)]`, `#[derive(Response)]`, `#[derive(ParamFilter)]`
- `set_if_some!` macro for conditional assignment

### Tests
- Inline `mod tests` at file bottom
- Sync: `#[test] fn test_name()`
- Async: `#[tokio::test] async fn test_name()`
- Panic: `#[test] #[should_panic(expected = "...")]`
- No integration test dirs; tests co-located with source

### Logging
- Use `tracing` crate: `debug!()`, `warn!()`, `error!()`, `info!()`
- OpenTelemetry integration

## Common Workflows

### Build
- Entire workspace: `cargo build`
- Specific crate: `cargo build -p <crate>`

### Test
- All tests: `cargo test`
- Single crate: `cargo test -p <crate>`
- Single test: `cargo test -p <crate> <test_name>`
- Async tests: `cargo test -p <crate> -- --test-threads=1`

### Lint & Format
- Lint: `cargo clippy`
- Format: `cargo fmt`

### Migrations
- Run: `cargo run --bin migrations_<feature> -- -v -u <db_url> -s <schema>`
- Rollback: `... down`
- Status: `... status`

### Start Services
- Infra: `bash start-service.sh`
- All microservices: `bash start.sh`
- Kill all: `bash start.sh kill`
- Docker infra: `docker-compose -f docker-compose.no_api.yml up -d`

## Agent/AI Usage Guidelines

- Follow conventions above for all generated code
- Use procedural macros for DTOs, queries, mutations, and filters
- Always add OpenAPI docs to handlers
- Place tests at the bottom of source files
- Use explicit error types and wrap errors as `AppError`
- Use tracing and logging macros for observability

## References
- See AGENTS.md for full conventions
- See apis/<name>/README.md for service-specific details
- See features/<name>/README.md for feature-specific details
- See test.rest files for example API requests

---

This SKILL.md is intended as a quick reference for both AI agents and developers working in the dn-ms Rust monorepo. For deeper details, consult AGENTS.md and service/feature READMEs.
