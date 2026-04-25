# Query Macro

The `#[derive(Query)]` macro generates a full `QueryManager` trait implementation for a SeaORM entity, including filtering, pagination, ordering, and related entity loading.

**Location:** `libs/shared/shared/macro/src/query.rs`
**Trait:** `shared_shared_data_core::query::QueryManager`

---

## Attributes

### `#[query(key_type(...))]`

Specifies the primary key type. Determines which `get_by_id_*` variant is implemented.

| Value    | Implements                |
|----------|---------------------------|
| `Uuid`   | `get_by_id_uuid`          |
| `i32`    | `get_by_id_i32`           |
| `String` | `get_by_id_str`           |

### `#[query_filter(column_name(...))]`

Specifies the SeaORM `Column` enum to use for building filter conditions. Generates `filter_condition_<column_name>` which handles all `FilterEnum` variants (`String`, `Bool`, `I32`, `U32`, `I64`, `U64`, `F32`, `F64`, `Uuid`, `VecString`, `DateTime`).

### `#[query_related(entity(...), field(...), name("..."))]`

Defines a related entity for eager loading. Can be specified multiple times for multiple relations.

| Parameter | Description |
|-----------|-------------|
| `entity`  | The related SeaORM `Entity` type (e.g., `ItemEntity`) |
| `field`   | The field name on `ModelOptionDto` to populate (e.g., `items`) |
| `name`    | The string clients pass in `includes` to request this relation (e.g., `"items"`) |

When `query_related` is present, the macro generates:
- `filter_with_related_entities` — paginated list query that loads related entities for each result
- `get_by_id_*_with_related_entities` — single entity query that loads related entities by ID

Both methods accept `includes: &Vec<String>` and only load relations whose `name` appears in the list.

---

## Required Implementations

The user must implement `build_filter_condition` on the struct:

```rust
impl MyQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}
```

---

## Generated Methods

| Method | Description |
|--------|-------------|
| `get_by_id_uuid(id)` | Find entity by UUID primary key |
| `get_by_id_i32(id)` | Find entity by i32 primary key |
| `get_by_id_str(id)` | Find entity by String primary key |
| `get_by_id_uuid_with_related_entities(id, &includes)` | Find by UUID + load related entities |
| `get_by_id_i32_with_related_entities(id, &includes)` | Find by i32 + load related entities |
| `get_by_id_str_with_related_entities(id, &includes)` | Find by String + load related entities |
| `filter(pagination, order, filters)` | Paginated filtered query |
| `filter_with_related_entities(pagination, order, filters, &includes)` | Paginated filtered query + load related entities |
| `get_db()` | Returns the read database connection |
| `build_query(order, filters)` | Builds the base `Select` with ordering and filter conditions |

---

## Full Example

```rust
use shared_shared_macro::Query;
use features_lookup_entities::lookup_item::Entity as ItemEntity;
use features_lookup_entities::lookup_type::{ActiveModel, Column, Entity, ModelOptionDto};

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(ItemEntity), field(items), name("items"))]
struct LookupTypeQueryManager;

impl LookupTypeQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}
```

### Usage in repo layer

```rust
// Get by ID with related entities
let includes = query_params.includes();
let model = LookupTypeQueryManager::get_by_id_uuid_with_related_entities(id, &includes).await?;

// Paginated list with related entities
let result = LookupTypeQueryManager::filter_with_related_entities(
    &pagination, &order, &filters, &includes
).await?;

// Simple query without relations
let model = LookupTypeQueryManager::get_by_id_uuid(id).await?;
let result = LookupTypeQueryManager::filter(&pagination, &order, &filters).await?;
```

---

## QueryParams

The `QueryParams` struct (`shared_shared_data_core::query_params`) deserializes the `includes` query parameter from the client:

```
GET /lookup-types/123?includes=items
GET /lookup-types?includes=items,categories
GET /lookup-types  (no includes — defaults to empty vec)
```

Pass `&QueryParams` through the layers (API → service → repo) and call `.includes()` to get the `Vec<String>`.

---

## Dependencies

The macro expects these items in scope (imported automatically by the generated code):
- `Entity`, `ActiveModel`, `Column`, `ModelOptionDto` from the entity crate
- `Pagination`, `Order`, `FilterEnum`, `QueryResult` from `shared_shared_data_core`
- `DB_READ` from `shared_shared_config::db`
