# API Lookup Service

A microservice for managing lookup data (reference data / master data) with multi-language translation support. This service provides a centralized way to manage and query lookup types, items, and their translations across multiple locales.

## Overview

The Lookup Service is part of the dn-ms microservices architecture and serves as a reference data management system. It manages:

- **Lookup Types**: Categories of reference data (e.g., Currency, Status, Country)
- **Lookup Items**: Individual entries within a type (e.g., USD, EUR, EUR for Currency type)
- **Item Translations**: Multi-language translations for lookup items

## Features

- ✅ CRUD operations for lookup types and items
- ✅ Multi-language translation support
- ✅ Pagination and filtering
- ✅ OpenAPI/Swagger documentation
- ✅ Distributed tracing with OpenTelemetry
- ✅ PostgreSQL with Sea-ORM
- ✅ Redis caching support
- ✅ Full RESTful API

## Prerequisites

- Rust 1.85.0+
- PostgreSQL 12+
- Redis (optional for caching)
- Docker & Docker Compose (for running infrastructure)

## Build & Run

### Build the Service

```bash
# Build the entire workspace
cargo build

# Build only this service
cargo build -p api-lookup

# Build optimized release
cargo build -p api-lookup --release
```

### Run the Service

```bash
# Using cargo run
cargo run -p api-lookup

# Using the compiled binary
./target/debug/api-lookup
./target/release/api-lookup

# Using environment variables for configuration
PORT=5161 LOOKUP_DATABASE_URL="postgresql://user:pass@localhost/db" cargo run -p api-lookup
```

### Start with Infrastructure

```bash
# Start all infrastructure services
bash start-service.sh

# Start all microservices (including lookup)
bash start.sh

# Kill all services
bash start.sh kill
```

## Configuration

The service uses environment variables for configuration via the `AppConfig` system and Consul for service discovery.

### Key Configuration Points

1. **Database**: Uses separate read/write URLs for database scaling
2. **Caching**: Redis for distributed caching with authentication
3. **Events**: Kafka integration for event-driven architecture
4. **Logging**: OpenTelemetry integration with structured logging
5. **Service Discovery**: Consul for dynamic service registration

### Environment Variables

| Variable | Example Value | Description |
|----------|---------------|-------------|
| `LOOKUP_PORT` | `5161` | HTTP server port (uses `INVENTORY_PORT` value if set) |
| `LOOKUP_DATABASE_READ_URL` | `postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms` | PostgreSQL connection string (read operations) |
| `LOOKUP_DATABASE_WRITE_URL` | `postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms` | PostgreSQL connection string (write operations) |
| `LOOKUP_DATABASE_SCHEME` | `lookup` | Database schema name |
| `LOOKUP_REDIS_URL` | `redis://:Redis!123@localhost:6379` | Redis connection string with password |
| `LOOKUP_CONSUMER_EVENT_KAFKA_BOOTSTRAP_SERVERS` | `localhost:9092` | Kafka bootstrap servers for event consumption |
| `LOOKUP_CONSUMER_EVENT_KAFKA_TOPIC` | `event_topic` | Kafka topic to consume events from |
| `RUST_LOG` | `debug,otel::tracing=trace` | Logging level (debug, info, warn, error) |
| `RUST_LOG_DIRECTORY` | `./logs` | Directory for log files |
| `CONSUL_HTTP_ADDR` | `http://127.0.0.1:8500` | Consul service discovery address |

### Database Connection

The service uses separate read and write database URLs configured via environment variables.

PostgreSQL connection string format:
```
postgresql://username:password@host:port/database
```

Example configuration from `.env`:
```
LOOKUP_DATABASE_READ_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
LOOKUP_DATABASE_WRITE_URL=postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms
LOOKUP_DATABASE_SCHEME=lookup
```

The schema name is set via `LOOKUP_DATABASE_SCHEME` environment variable.

### Running Migrations

```bash
# Run migrations
cargo run --bin migrations_lookup -- -v -u "postgresql://dn_ms:password123@localhost:5432/dn_ms" -s lookup

# Check migration status
cargo run --bin migrations_lookup -- -v -u "postgresql://dn_ms:password123@localhost:5432/dn_ms" -s lookup status

# Rollback migrations
cargo run --bin migrations_lookup -- -v -u "postgresql://dn_ms:password123@localhost:5432/dn_ms" -s lookup down
```

## API Endpoints

### Health Check

```http
GET /healthchecker
```

### Swagger Documentation

```http
GET /swagger-ui/
```

### Lookup Types

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lookup-types` | List all lookup types with pagination |
| POST | `/lookup-types` | Create a new lookup type |
| GET | `/lookup-types/{code}` | Get a specific lookup type by code |
| PATCH | `/lookup-types/{code}` | Update a lookup type |
| DELETE | `/lookup-types/{code}` | Delete a lookup type |

### Lookup Items

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lookup-types/{type_code}/items` | List items of a type with pagination |
| POST | `/lookup-types/{type_code}/items` | Create a new item |
| GET | `/lookup-types/{type_code}/items/{id}` | Get a specific item by ID |
| PATCH | `/lookup-types/{type_code}/items/{id}` | Update an item |
| DELETE | `/lookup-types/{type_code}/items/{id}` | Delete an item |

### Lookup Item Translations

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lookup-types/{type_code}/items/{item_id}/translations` | List all translations for an item |
| POST | `/lookup-types/{type_code}/items/{item_id}/translations` | Create a translation |
| PATCH | `/lookup-types/{type_code}/items/{item_id}/translations/{locale}` | Update a translation by locale |
| DELETE | `/lookup-types/{type_code}/items/{item_id}/translations/{locale}` | Delete a translation by locale |

## Testing

### Using REST Client

Use the provided `test.rest` file with VS Code's REST Client extension:

```bash
# Open the file
code test.rest

# Execute requests directly from the editor (Ctrl+Alt+R on each request)
```

### Run Tests

```bash
# Run all tests for this service
cargo test -p api-lookup

# Run tests with output
cargo test -p api-lookup -- --nocapture

# Run a specific test
cargo test -p api-lookup test_name
```

### Example Requests

**Create a Lookup Type:**
```bash
curl -X POST http://localhost:5161/lookup-types \
  -H "Content-Type: application/json" \
  -d '{
    "code": "currency",
    "description": "Currency types"
  }'
```

**Create a Lookup Item:**
```bash
curl -X POST http://localhost:5161/lookup-types/currency/items \
  -H "Content-Type: application/json" \
  -d '{
    "value": "USD",
    "label": "US Dollar",
    "display_order": 1,
    "is_active": true
  }'
```

**Create a Translation:**
```bash
curl -X POST "http://localhost:5161/lookup-types/currency/items/{item_id}/translations" \
  -H "Content-Type: application/json" \
  -d '{
    "locale": "vi",
    "label": "Đô la Mỹ",
    "description": "Tiền tệ của Mỹ"
  }'
```

## Project Structure

```
apis/lookup/
├── Cargo.toml                          # Package manifest
├── README.md                           # This file
├── test.rest                           # REST API test file
└── src/
    ├── main.rs                         # Entry point
    ├── app.rs                          # Application setup
    ├── doc.rs                          # OpenAPI documentation
    └── routes/
        ├── lookup_type.rs              # Lookup Type routes
        ├── lookup_item.rs              # Lookup Item routes
        └── lookup_item_translation.rs   # Translation routes
```

## Dependencies

### Shared Libraries

- **shared-shared-app**: Generic application startup and configuration
- **shared-shared-data-app**: Common data structures and response types
- **shared-shared-data-core**: Pagination, filtering, and ordering
- **shared-shared-data-error**: Error handling and types
- **shared-shared-extractor**: Custom Axum request extractors
- **shared-shared-middleware**: Middleware implementations
- **shared-shared-observability**: OpenTelemetry integration

### Feature Libraries

- **features-lookup-model**: Data models and DTOs
- **features-lookup-service**: Business logic layer
- **features-lookup-migrations**: Database migrations

### External Crates

- **axum**: Web framework
- **tokio**: Async runtime
- **utoipa**: OpenAPI/Swagger support
- **uuid**: UUID generation
- **tracing**: Structured logging

## Architecture

### Layered Architecture

```
Request → Router (routes/) → Handler → Service → Repository → Database
          ↓
          Middleware/Extractors
          ↓
          Error Handling
          ↓
          Response
```

### Event-Driven Architecture

The service integrates with Kafka for event consumption:

- **Kafka Topic**: Consumes from `event_topic`
- **Bootstrap Servers**: Configured via `LOOKUP_CONSUMER_EVENT_KAFKA_BOOTSTRAP_SERVERS`
- **Use Case**: Reacts to domain events for data synchronization and cache invalidation

### Caching Strategy

- **Redis Cache**: Configured via `LOOKUP_REDIS_URL` with authentication
- **TTL-Based Expiration**: Automatic cache expiration for stale data
- **Cache Invalidation**: Triggered by Kafka events or direct API mutations

### Handler Pattern

1. **Routes** (`routes/*`): HTTP endpoint definitions with OpenAPI docs
2. **Service** (`features-lookup-service`): Business logic layer
3. **Repository** (`features-lookup-repo`): Data access layer
4. **Model** (`features-lookup-model`): Data structures and DTOs

## Error Handling

The service uses structured error handling via `AppError` with `#[from]` attribute for automatic conversion:

```rust
pub type Result<T> = core::result::Result<T, AppError>;
```

Common error responses:

| Status | Error | Description |
|--------|-------|-------------|
| 400 | `Bad Request` | Invalid request parameters |
| 404 | `Not Found` | Resource not found |
| 409 | `Conflict` | Resource conflict (e.g., duplicate code) |
| 500 | `Internal Server Error` | Unexpected error |

## Logging & Observability

The service integrates with OpenTelemetry for distributed tracing:

- **Structured Logging**: Using `tracing` crate
- **Trace Instrumentation**: `#[instrument]` macro on handlers and services
- **Log Levels**: trace, debug, info, warn, error
- **OTEL Export**: Metrics and traces exported to OTEL collector

### Configuring Logs

Use the `RUST_LOG` environment variable to control logging granularity:

```bash
# Example with detailed tracing
RUST_LOG=debug,otel::tracing=trace,axum_tracing_opentelemetry=trace,sea_orm::driver::sqlx_postgres=info

# Simple debug logging
RUST_LOG=debug

# Production-level logging
RUST_LOG=info
```

Logs are written to the directory specified by `RUST_LOG_DIRECTORY` (default: `./logs`)

## Development

### Code Style

- Rust Edition 2021
- 4-space indentation
- Follow AGENTS.md conventions for naming and imports
- Use `#[instrument]` for tracing on async functions
- Use `#[derive(Dto)]` and related macros for code generation

### Adding New Endpoints

1. Define route handler in `routes/*.rs`
2. Add `#[utoipa::path(...)]` attribute for OpenAPI docs
3. Add `#[instrument(...)]` for tracing
4. Implement business logic in service layer
5. Add tests in the same file with `#[cfg(test)]` module
6. Register route in `app.rs`

### Running Linter

```bash
cargo clippy -p api-lookup
```

### Format Code

```bash
cargo fmt -p api-lookup
```

## Performance

### Service Discovery

The service integrates with Consul for dynamic service registration and discovery:

```
CONSUL_HTTP_ADDR=http://127.0.0.1:8500
```

This enables:
- Automatic service registration
- Health checking
- DNS-based service discovery across the microservices network

### Caching Strategy

- Redis caching with authentication (`redis://:Redis!123@localhost:6379`)
- TTL-based expiration for cache entries
- Automatic cache invalidation on updates via Kafka events

### Query Optimization

- Pagination for large result sets (via `QueryResult` wrapper)
- Indexed columns in PostgreSQL
- Connection pooling via Sea-ORM
- Separate read/write database URLs for scaling

## Support

For issues or questions:
1. Check the `test.rest` file for example requests
2. Review `AGENTS.md` for project conventions
3. Check service logs in `logs/lookup/`
4. Review OpenAPI documentation at `/swagger-ui/`

## License

Part of the dn-ms microservices architecture.
