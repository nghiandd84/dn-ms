# Lookup Service

Manages lookup tables and reference data (e.g., currencies, countries, status enums) for all services in the platform.

## Responsibilities
- CRUD for lookup types (categories), items (values), and item translations (i18n)
- Multi-tenant data isolation via `tenant_id`
- Filtering, pagination, ordering on all list endpoints
- Eager-loading related entities (e.g., `?includes=items`)
- Seeding reference data (countries) via migrations

## Crate Structure
| Crate | Path | Purpose |
|-------|------|---------|
| features-lookup-entities | `features/lookup/entities` | SeaORM entity models (lookup_type, lookup_item, lookup_item_translation) |
| features-lookup-model | `features/lookup/model` | DTOs, request/response types, filter params, app state |
| features-lookup-repo | `features/lookup/repo` | Query (filtered list, get-by-id, get-by-code) and Mutation (create/update/delete) |
| features-lookup-service | `features/lookup/service` | Thin service layer delegating to repo |
| features-lookup-migrations | `features/lookup/migrations` | SeaORM migrations for schema and seed data |
| apis/lookup | `apis/lookup` | Axum API routes, middleware, Swagger UI |

## Key Patterns
- **Derive macros**: `#[derive(Query)]`, `#[derive(Mutation)]`, `#[derive(Dto)]`, `#[derive(Response)]`, `#[derive(ParamFilter)]` generate boilerplate for filtering, CRUD, and DTO conversion
- **TenantId extractor**: Extracts tenant from request headers, injected into create payloads and list filters
- **ValidJson**: Axum extractor that validates request body via `validator` crate before handler runs
- **FilterParams**: Deserializes query string filters (e.g., `?code=li|CURRENCY`) into typed `FilterEnum` values
- **Consul discovery**: Periodic permission sync from auth service via Consul

## API Surface
- `POST/GET /lookup-types`, `GET/PATCH/DELETE /lookup-types/{id}`
- `POST/GET /lookup-types/{type_code}/items`, `GET/PATCH/DELETE /lookup-types/{type_code}/items/{item_id}`
- `POST/GET /lookup-types/{type_code}/items/{item_id}/translations`, `PATCH/DELETE .../translations/{id}`

## Consumed By
- All services — for reference data validation and display values
