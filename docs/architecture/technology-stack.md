# Technology Stack

This document summarizes the core technologies, frameworks, and tools used in the dn-ms Rust monorepo project.

---


## Programming Language
- **Rust** (2021 edition, 1.85.0+)
	- Strong type safety, async-first, zero-cost abstractions
	- Used for all backend microservices, shared libraries, and CLI tools


## Web Framework
- **Axum** — HTTP server, routing, extractors, middleware
	- Modular, async, and built on top of Tokio
	- Used for all API entrypoints and middleware


## Database & ORM
- **PostgreSQL** — Primary relational database
	- Used for all persistent data storage
- **SeaORM** — Async ORM for Rust (models, queries, migrations)
	- Entity modeling, migrations, and query abstraction
	- Each service has its own migration binary


## Messaging & Streaming
- **Kafka** — Event streaming and message broker (KRaft mode)
	- Used for inter-service event-driven communication
	- Kafka consumers implemented in feature/stream/


## Caching
- **Redis** — High-performance cache and distributed locking
	- Used for caching, distributed locks, and idempotency


## Service Discovery & Configuration
- **Consul** — Service discovery, health checks, and KV configuration
	- Used for service registration, discovery, and dynamic config


## Observability & Tracing
- **OpenTelemetry** — Distributed tracing and metrics
	- Instrumentation for all services
- **Jaeger** — Tracing backend and UI
	- Visualizes distributed traces
- **OpenObserve** — Log and trace aggregation (optional)
	- Centralized log and trace storage
- **tracing** — Structured logging for Rust
	- Used throughout all code for diagnostics


## Containerization & Orchestration
- **Docker** — Containerization for local development and CI
- **Docker Compose** — Multi-service orchestration for infrastructure
	- Used to spin up PostgreSQL, Redis, Kafka, Consul, etc.


## Build & Dev Tools
- **Cargo** — Rust package manager and build tool
	- Used for building, testing, and running all crates
- **tokio** — Async runtime for Rust
	- Foundation for all async code
- **thiserror** — Error handling
	- Standardized error enums and conversions
- **async-trait** — Async trait support
	- Enables async in trait definitions
- **utoipa** — OpenAPI documentation generation
	- Auto-generates OpenAPI/Swagger docs from code
- **tracing** — Logging and instrumentation
- **rustfmt** — Code formatting
- **clippy** — Linting
- **dotenv** — Loads environment variables from .env files
- **uuid** — Universally unique identifiers for primary keys
- **chrono** — Date and time handling
- **serde** — Serialization/deserialization for JSON, etc.


## API Documentation & Testing
- **OpenAPI/Swagger** — API specs (via utoipa)
- **VS Code REST Client** — Manual API testing (`test.rest` files)
	- Each apis/<service>/test.rest file contains sample requests


## Frontend (Apps)
- **Tailwind CSS** — Utility-first CSS framework (for auth-web)
- **JavaScript/TypeScript** — For web UIs (auth-web, etc.)
	- Used in apps/auth-web and other frontend apps


## Other Libraries & Utilities
- **uuid** — Unique identifiers
- **chrono** — Date and time handling
- **serde** — Serialization/deserialization
- **dotenv** — Environment variable management
- **humao.rest-client** — VS Code extension for REST API testing

---


---

## Integration Notes
- All microservices follow the same layered structure for maintainability.
- Shared libraries and macros ensure consistency and reduce duplication.
- Infrastructure is managed via scripts and Docker Compose for reproducibility.
- Observability is built-in from the start (tracing, metrics, logs).
- API documentation is auto-generated and kept in sync with code.

## References
- See [layered-architecture.md](layered-architecture.md) for how these technologies fit into the system design.
- See [overview-diagram.md](overview-diagram.md) for a visual overview.
