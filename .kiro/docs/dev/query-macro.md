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

When exactly one `#[query_filter]` is specified, the macro also auto-generates `build_filter_condition`. For multiple `#[query_filter]` attributes (e.g., cross-entity filtering), you must implement `build_filter_condition` manually.

### `#[query_related(entity(...), column(...), field(...), name("..."))]`

Defines a related entity for eager loading. Can be specified multiple times for multiple relations.

| Parameter | Description | Required |
|-----------|-------------|----------|
| `entity`  | The related SeaORM `Entity` type (e.g., `ItemEntity`) | Yes |
| `column`  | The related entity's `Column` enum for filtering (e.g., `PermissionColumn`) | No |
| `field`   | The field name on `ModelOptionDto` to populate (e.g., `items`) | Yes |
| `name`    | The string clients pass in `includes` to request this relation (e.g., `"items"`) | No (defaults to field) |

When `query_related` is present, the macro generates:
- `filter_with_related_entities` — paginated list query that loads related entities for each result
- `get_by_id_*_with_related_entities` — single entity query that loads related entities by ID

When `column` is specified, related filters narrow the **parent result set** via JOIN subquery (e.g., "only return roles that have a permission matching the filter").

---

## Auto-Generated vs Manual

### Auto-generated `build_filter_condition` (single `#[query_filter]`)

When there is exactly one `#[query_filter(column_name(Column))]`, the macro generates:

```rust
fn build_filter_condition(filter_condition: &FilterCondition) -> Condition {
    // Recursively walks And/Or/Leaf tree
    // Maps Leaf -> Column::from_str + filter_condition_column
}
```

No user code needed — just derive and go.

### Manual `build_filter_condition` (multiple `#[query_filter]`)

When there are multiple `#[query_filter]` attributes (cross-entity filtering), implement manually:

```rust
impl BakerQueryManager {
    fn build_filter_condition(filter_condition: &FilterCondition) -> Condition {
        match filter_condition {
            FilterCondition::And(conditions) => {
                let mut condition = Condition::all();
                for c in conditions { condition = condition.add(Self::build_filter_condition(c)); }
                condition
            }
            FilterCondition::Or(conditions) => {
                let mut condition = Condition::any();
                for c in conditions { condition = condition.add(Self::build_filter_condition(c)); }
                condition
            }
            FilterCondition::Leaf(filter_enum) => {
                let name = filter_enum.get_name();
                if name.starts_with("bakery.") {
                    if let Ok(col) = BakeryColumn::from_str(&name[7..]) {
                        return Self::filter_condition_bakerycolumn(col, filter_enum);
                    }
                } else if let Ok(col) = Column::from_str(name.as_str()) {
                    return Self::filter_condition_column(col, filter_enum);
                }
                Condition::all()
            }
        }
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
| `get_by_id_*_with_related_entities(id, &includes, &related_filters)` | Find by ID + load related entities |
| `filter(pagination, order, &filter_condition)` | Paginated filtered query |
| `filter_with_related_entities(pagination, order, &filter_condition, &includes, &related_filters)` | Paginated filtered query + load related entities |
| `build_query(order, &filter_condition)` | Builds the base `Select` with ordering and filter conditions |
| `paginate_query(pagination, order, &filter_condition)` | Paginate and fetch a page |
| `build_filter_condition(&filter_condition)` | Recursively builds SeaORM `Condition` from `FilterCondition` tree |
| `get_db()` | Returns the read database connection |

---

## filter_with_related_entities — 4-Step Flow

1. **Build base query** from parent filters + related filters (JOIN subquery narrows parents)
2. **Count total pages** on the filtered base query (no ordering for efficient count)
3. **Apply ordering**, paginate, fetch page, load related entities for the page
4. **Map to DTOs** preserving original order and return

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
// build_filter_condition is auto-generated — no manual impl needed
```

### With related entity filtering

```rust
use features_auth_entities::permission::Column as PermissionColumn;
use features_auth_entities::permission::Entity as PermissionEntity;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(
    entity(PermissionEntity),
    column(PermissionColumn),
    field(permissions),
    name("permissions")
)]
struct RoleQueryManager;
```

### Usage in repo layer

```rust
// Simple query
let condition = FilterCondition::from(filters); // Vec<FilterEnum> -> FilterCondition
let result = MyQueryManager::filter(&pagination, &order, &condition).await?;

// With related entities
let result = MyQueryManager::filter_with_related_entities(
    &pagination, &order, &condition, &includes, &related_filters
).await?;

// Get by ID with related
let model = MyQueryManager::get_by_id_uuid_with_related_entities(
    id, &includes, &related_filters
).await?;
```

---

## Macro Architecture

All derive macros in `libs/shared/shared/macro/src/lib.rs` follow the same pattern:

```rust
pub fn query_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let query_input = query::QueryInput::parse_from(derive_input);  // parsing
    query::query_impl(query_input)                                   // code generation
}
```

Each macro module defines:
- **Attribute parse types** — custom `syn::Parse` implementations (no `Meta::List`)
- **Input struct** — holds all parsed data (`QueryInput`, `MutationInput`, etc.)
- **`parse_from(DeriveInput)`** — extracts attributes into the input struct
- **`impl_fn(Input)`** — pure code generation, no parsing

| Macro | Input Struct | Module |
|-------|-------------|--------|
| `Query` | `QueryInput` | `query.rs` |
| `Mutation` | `MutationInput` | `mutation.rs` |
| `RemoteService` | `RemoteServiceInput` | `service.rs` |
| `Dto` | `DtoInput` | `dto.rs` |
| `ParamFilter` | `FilterInput` | `filter.rs` |
| `Response` / `ResponseGeneric` | `ResponseInput` | `response.rs` |

---

## Dependencies

The macro generates these imports automatically:
- `std::str::FromStr`
- `sea_orm::{Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, ...}`
- `shared_shared_data_core::{query::QueryManager, filter::FilterOperator, filter::FilterCondition, order::OrderDirection}`
- `shared_shared_config::db::DB_READ`
