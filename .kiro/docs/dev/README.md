# Developer Documentation

This folder contains guides for development environment setup, Rust patterns, and operational procedures.

## Contents

### Patterns & Systems
- [Permission System (RBAC)](permission-system.md) — Auth extractors, permission definitions, baggage header, ADMIN_ALL bypass, permission sync
- [Gateway Interceptors](gateway-interceptors.md) — Rate limiter, token auth, CORS, request ID interceptor system
- [Query Macro](query-macro.md) — `#[derive(Query)]` macro for auto-generated CRUD queries
- [FilterCondition AND/OR Logic](filter-condition.md) — Filter system for query parameters
- [RemoteService Pattern](remote-service.md) — HTTP client pattern for inter-service communication

### Setup & Operations
- [Setup Guide](setup-guide.md) — Development environment setup
- [Migrations](migrations.md) — Database migration procedures
- [Testing](testing.md) — Test framework and conventions
- [Infrastructure Installs](installs/summary.md) — Consul, Kafka, Redis, Postgres, OpenTelemetry setup
