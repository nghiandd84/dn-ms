# Layered Architecture

This project follows a layered microservices architecture, with each service and supporting library organized into clear layers. Below is an overview of the main layers and their responsibilities, as well as how they interact across the monorepo.

---

## Layer Overview

1. **API Layer (apis/)**
   - Entry point for HTTP requests (Axum routers, handlers, middleware)
   - Handles routing, authentication, request validation, and response formatting
   - Forwards requests to the service layer

2. **Feature Layer (features/<service>/)**
   - Contains all business logic and domain-specific code for each microservice
   - Organized into sub-layers:
     - **entities/**: SeaORM database models (structs, relations)
     - **model/**: DTOs, request/response types, state structs
     - **repo/**: Data access layer (queries, mutations, persistence)
     - **service/**: Business logic, orchestration, validation
     - **migrations/**: Database schema migrations
     - **stream/**: Kafka consumers and event processing
     - **remote/**: Remote service clients (for inter-service calls)

3. **Shared Libraries (libs/shared/)**
   - Common utilities, types, and abstractions used across all services
   - Submodules include:
     - **data/**: Result types, errors, pagination, cache, auth
     - **app/**: AppState, config, startup helpers
     - **observability/**: OpenTelemetry setup
     - **config/**: Configuration loading
     - **extractor/**: Axum request extractors
     - **middleware/**: Axum middleware
     - **macro/**: Procedural macros (Query, Mutation, Dto, Response, ParamFilter)
     - **macro-rule/**: Declarative macros (set_if_some!)
     - **auth/**: Permission system

4. **Tool Libraries (libs/tools/)**
   - Specialized tools and integrations (e.g., Consul client)

5. **Apps (apps/)**
   - Standalone applications (e.g., gateway, notification app, auth web)
   - May aggregate or proxy requests to multiple APIs

6. **Infrastructure (docker/, start-service.sh, etc.)**
   - Scripts and configs for running PostgreSQL, Redis, Kafka, Consul, OpenTelemetry, etc.

---

## Layered Flow Example

1. **Request**: Client sends HTTP request to an API endpoint (e.g., /wallets)
2. **API Layer**: Handler validates request, extracts data, and calls the service layer
3. **Service Layer**: Business logic is executed, possibly involving multiple repositories or remote calls
4. **Repo Layer**: Data is fetched or mutated in the database via SeaORM models
5. **Entities/Model**: Data is mapped to/from database structs and DTOs
6. **Response**: Result is returned up the stack, formatted, and sent to the client

---

## Diagram (Mermaid)

```mermaid
flowchart TD
    Client --> API[API Layer (Axum)]
    API --> Service[Service Layer]
    Service --> Repo[Repo Layer]
    Repo --> Entities[Entities/Model Layer]
    Service --> Remote[Remote Clients]
    API --> Middleware[Middleware/Extractors]
    API --> Shared[Shared Libraries]
    Service --> Shared
    Repo --> Shared
    API --> Observability[Observability]
    Service --> Observability
    API --> Config[Config]
    Service --> Config
    API --> Macro[Macros]
    Service --> Macro
    API --> Auth[Auth]
    Service --> Auth
    API --> ToolLibs[Tool Libraries]
    Service --> ToolLibs
    API --> Apps[Apps]
    Service --> Apps
    API --> Infra[Infrastructure]
    Service --> Infra
```

---

## Notes
- Each microservice is self-contained but follows the same layered pattern.
- Shared libraries and tools promote code reuse and consistency.
- Infrastructure scripts and configs are versioned with the codebase for reproducibility.
