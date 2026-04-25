# Lookup Service API

Manages lookup tables (types), their items, and item translations. Provides reference data (e.g., currencies, countries, status enums) consumed by all other services.

## Main Responsibilities
- CRUD for lookup types (categories of reference data)
- CRUD for lookup items (values within a type)
- CRUD for lookup item translations (i18n for item names)
- Multi-tenant isolation via `tenant_id` extracted from request context
- Filtering, pagination, ordering, and eager-loading of related entities

## Base URL
`http://localhost:5161`

## Key API Groups & Endpoints

### Lookup Types (`/lookup-types`)

- `POST /lookup-types` — Create a lookup type
- `GET /lookup-types` — List lookup types (with filters, pagination, ordering, includes)
- `GET /lookup-types/{id}` — Get a lookup type by UUID
- `PATCH /lookup-types/{id}` — Update a lookup type
- `DELETE /lookup-types/{id}` — Delete a lookup type

### Lookup Items (`/lookup-types/{type_code}/items`)

- `POST /lookup-types/{type_code}/items` — Create a lookup item under a type
- `GET /lookup-types/{type_code}/items` — List items for a type (with filters, pagination, ordering)
- `GET /lookup-types/{type_code}/items/{item_id}` — Get item by UUID
- `PATCH /lookup-types/{type_code}/items/{item_id}` — Update an item
- `DELETE /lookup-types/{type_code}/items/{item_id}` — Delete an item

### Lookup Item Translations (`/lookup-types/{type_code}/items/{item_id}/translations`)

- `POST /lookup-types/{type_code}/items/{item_id}/translations` — Create a translation
- `GET /lookup-types/{type_code}/items/{item_id}/translations` — List translations
- `PATCH /lookup-types/{type_code}/items/{item_id}/translations/{translation_id}` — Update a translation
- `DELETE /lookup-types/{type_code}/items/{item_id}/translations/{translation_id}` — Delete a translation

## Data Models

### LookupTypeData (Response)
| Field        | Type           | Description                    |
|-------------|----------------|--------------------------------|
| id          | UUID           | Primary key                    |
| tenant_id   | String         | Tenant identifier              |
| code        | String         | Unique code (per tenant)       |
| name        | String         | Display name                   |
| description | String         | Description text               |
| is_active   | bool           | Active flag                    |
| created_at  | DateTime       | Creation timestamp             |
| updated_at  | DateTime       | Last update timestamp          |
| items       | Vec\<LookupItemData\> | Related items (when `includes=items`) |

### LookupTypeForCreateRequest
| Field       | Type           | Validation                     |
|-------------|----------------|--------------------------------|
| code        | String         | 1–50 chars, required           |
| name        | String         | 1–100 chars, required          |
| description | String?        | max 500 chars, optional        |
| tenant_id   | (injected)     | Set from TenantId extractor    |

### LookupTypeForUpdateRequest
| Field       | Type           | Validation                     |
|-------------|----------------|--------------------------------|
| code        | String?        | 1–50 chars                     |
| name        | String?        | 1–100 chars                    |
| description | String?        | max 500 chars                  |
| is_active   | bool?          | optional                       |

## Query Parameters

| Param           | Description                                      | Example                              |
|-----------------|--------------------------------------------------|--------------------------------------|
| page            | Page number (pagination)                         | `?page=1`                            |
| page_size       | Items per page                                   | `?page_size=20`                      |
| order_name      | Column to order by                               | `?order_name=code`                   |
| order_direction | 0 = ASC, 1 = DESC                               | `?order_direction=1`                 |
| includes        | Eager-load related entities                      | `?includes=items`                    |
| {column}        | Filter by column (supports operators like `li|`) | `?code=li|CURRENCY`                  |

## Architecture Layers

```
API (apis/lookup/src/routes/lookup_type.rs)
  → Service (features/lookup/service/src/lookup_type.rs)
    → Repo Query/Mutation (features/lookup/repo/src/lookup_type/)
      → Entity (features/lookup/entities/src/lookup_type.rs)
```

- **Entity**: SeaORM model on `lookup_types` table. UUID PK with `before_save` hook for auto-ID and timestamps. Has `has_many` relation to `lookup_items`.
- **Model**: DTOs — `LookupTypeData` (response), `LookupTypeForCreateRequest` / `LookupTypeForUpdateRequest` (input with validation). `ParamFilter` derive generates `LookupTypeDataFilterParams` for query filtering.
- **Repo**: `LookupTypeQuery` handles filtered listing (with optional related entity loading), get-by-id (with optional includes), get-by-code. `LookupTypeMutation` handles create/update/delete via `#[derive(Mutation)]` macro.
- **Service**: Thin layer (`LookupTypeService`) delegating to repo, mapping errors to `AppError`.
- **API Routes**: Axum handlers with `TenantId` extractor, `ValidJson` for validated input, `FilterParams` for query filters. Registered under `Router` with `AppState<LookupAppState, LookupCacheState>`.

## Multi-Tenancy
- `tenant_id` is extracted from request headers via `TenantId` extractor
- Injected into create requests and used as a filter on list queries
- Unique constraint on `(tenant_id, code)` ensures code uniqueness per tenant

## Integration Points
- Used by all services for reference data validation and display
- Consul-based service discovery with periodic permission sync from auth service

## Example Usage

```http
### Create a lookup type
POST /lookup-types
Content-Type: application/json

{
  "code": "CURRENCY",
  "name": "Lookup Currency",
  "description": "Currency types"
}

### List with filter and includes
GET /lookup-types?code=li|CURRENCY&includes=items

### Update
PATCH /lookup-types/{id}
Content-Type: application/json

{
  "description": "Updated currency types"
}
```
