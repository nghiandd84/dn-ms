# Tagging Service API

Centralized tagging microservice that manages tags organized in a two-level hierarchy (Tag Groups → Sub-Groups → Tags), supports multi-entity associations, and provides search/autocomplete, usage counts, tag merge/alias, and bulk tagging operations.

## Location

- API crate: `apis/tagging/`
- Feature crates: `features/tagging/{entities,model,repo,service,migrations}`

## Database Schema

### Tables
- `tag_groups` — Hierarchical tag categories (max 2 levels: group → sub-group)
- `tags` — Individual tags belonging to a group
- `entity_tags` — Polymorphic association table linking tags to any entity

### Key Constraints
- `tag_groups`: Unique `(tenant_id, code)`, self-ref FK on `parent_id`
- `tags`: Unique `(tenant_id, slug)`, FK to `tag_groups`, self-ref FK on `alias_of`
- `entity_tags`: Unique `(tag_id, entity_type, entity_id)`, FK to `tags`

## Endpoints

### Tag Groups
- `GET /tag-groups` — List tag groups with pagination/filters (Auth: `CanReadTagGroup`)
- `GET /tag-groups/{id}` — Get tag group by ID (Auth: `CanReadTagGroup`)
- `POST /tag-groups` — Create tag group (Auth: `CanCreateTagGroup`)
- `PATCH /tag-groups/{id}` — Update tag group (Auth: `CanUpdateTagGroup`)
- `DELETE /tag-groups/{id}` — Delete tag group (Auth: `CanDeleteTagGroup`)

### Tags
- `GET /tags` — List tags with pagination/filters (Auth: `CanReadTag`)
- `GET /tags/search?search=...&limit=N` — Autocomplete search (Auth: `CanReadTag`)
- `GET /tags/{id}` — Get tag by ID (Auth: `CanReadTag`)
- `POST /tags` — Create tag (Auth: `CanCreateTag`)
- `PATCH /tags/{id}` — Update tag (Auth: `CanUpdateTag`)
- `DELETE /tags/{id}` — Delete tag (Auth: `CanDeleteTag`)
- `GET /tags/{id}/usage` — Get tag usage count (Auth: `CanReadTag`)
- `POST /tags/{id}/merge` — Merge tag into another (Auth: `CanUpdateTag`)

### Entity Tags
- `POST /entity-tags` — Assign tag to entity (Auth: `CanCreateEntityTag`)
- `POST /entity-tags/bulk` — Bulk assign tags (Auth: `CanCreateEntityTag`)
- `POST /entity-tags/bulk-remove` — Bulk remove tags (Auth: `CanDeleteEntityTag`)
- `GET /entities/{entity_type}/{entity_id}/tags` — Get tags for entity (Auth: `CanReadEntityTag`)
- `GET /tags/{tag_id}/entities` — Get entities for tag (Auth: `CanReadEntityTag`)

## Request/Response Examples

### Create Tag Group
```json
POST /tag-groups
{
  "code": "GENRE",
  "name": "Genre",
  "description": "Music/event genres",
  "sort_order": 1
}
```

### Create Sub-Group
```json
POST /tag-groups
{
  "code": "MUSIC_GENRE",
  "name": "Music Genre",
  "description": "Specific music genres",
  "parent_id": "uuid-of-parent-group",
  "sort_order": 1
}
```

### Create Tag
```json
POST /tags
{
  "tag_group_id": "uuid-of-group",
  "name": "Rock",
  "slug": "rock",
  "color": "#FF5733",
  "description": "Rock music genre",
  "sort_order": 1
}
```

### Assign Tag to Entity
```json
POST /entity-tags
{
  "tag_id": "uuid-of-tag",
  "entity_type": "event",
  "entity_id": "uuid-of-event"
}
```

### Bulk Assign Tags
```json
POST /entity-tags/bulk
{
  "tag_ids": ["uuid-1", "uuid-2"],
  "entity_type": "event",
  "entity_ids": ["entity-uuid-1", "entity-uuid-2"]
}
```

Response:
```json
{
  "ok": true,
  "created": 3,
  "skipped": 1
}
```

### Merge Tag
```json
POST /tags/{source_tag_id}/merge
{
  "target_tag_id": "uuid-of-canonical-tag"
}
```

Merging reassigns all entity_tags from source to target and marks source as an alias (inactive).

### Search/Autocomplete
```
GET /tags/search?search=rock&limit=5
```

Returns active, non-alias tags matching the search term (case-insensitive contains).

## Permissions

| Permission | Resource | Action |
|-----------|----------|--------|
| CanCreateTagGroup | TAGGING:TAG_GROUP | CREATE |
| CanReadTagGroup | TAGGING:TAG_GROUP | READ |
| CanUpdateTagGroup | TAGGING:TAG_GROUP | UPDATE |
| CanDeleteTagGroup | TAGGING:TAG_GROUP | DELETE |
| CanCreateTag | TAGGING:TAG | CREATE |
| CanReadTag | TAGGING:TAG | READ |
| CanUpdateTag | TAGGING:TAG | UPDATE |
| CanDeleteTag | TAGGING:TAG | DELETE |
| CanCreateEntityTag | TAGGING:ENTITY_TAG | CREATE |
| CanReadEntityTag | TAGGING:ENTITY_TAG | READ |
| CanDeleteEntityTag | TAGGING:ENTITY_TAG | DELETE |

## Business Rules

### Hierarchy Depth
- Maximum 2 levels: Tag Group → Sub-Group → Tags
- A sub-group cannot have children (enforced on create/update)
- A tag group cannot be its own parent

### Tag Aliases
- When a tag is merged into another, it becomes an alias (`alias_of` points to canonical tag)
- Aliased tags are marked inactive
- When assigning a tag, if the tag is an alias, the canonical tag is used instead

### Usage Counts
- `usage_count` on the `tags` table is a denormalized counter
- Incremented on tag assignment, decremented on removal
- `GET /tags/{id}/usage` returns the live count from `entity_tags` table

### Tenant Scoping
- All entities are scoped by `tenant_id`
- Tags from one tenant are not visible to another
- The `tenant_id` is extracted from the baggage header

### Entity Types
Valid `entity_type` values are arbitrary strings (e.g., `event`, `merchant`, `booking`, `inventory`). No enum restriction — any service can use its own entity type.

## Configuration (.env)

```
TAGGING_REDIS_URL=redis://:Redis!123@localhost:6379
TAGGING_DATABASE_READ_URL=${DATABASE_URL}
TAGGING_DATABASE_WRITE_URL=${DATABASE_URL}
TAGGING_DATABASE_SCHEME=tagging
TAGGING_PORT=5211
```

## Infrastructure

- **Ports**: 5211 (instance 1), 5212 (instance 2)
- **Gateway path**: `/api/tagging`
- **Consul service name**: `tagging`
- **Service key**: `TAGGING`

## Query Parameters

Supports standard query patterns:
- `?includes=tag_group` — Include related tag group in tag responses
- `?fields=id,name,slug` — Select specific fields
- `?name=li|rock` — Filter by name (LIKE)
- `?tenant_id=eq|my-tenant` — Filter by tenant
- `?tag_group_id=eq|uuid` — Filter tags by group
- `?is_active=eq|true` — Filter active only
- `?order_name=name&order_direction=1` — Sort ascending by name
