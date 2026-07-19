# Field Permission Configuration Guide

How to configure field-level access control (FLAC) for roles on resources.

## Overview

Field permissions control which fields of a resource a role can READ or UPDATE. They work alongside existing resource-level permissions (RBAC).

**Prerequisites:**
- The role must already have resource-level permission (via `role_permissions` table with correct mask bit)
- Field permissions add **field-level granularity** on top of resource-level access

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/field-permissions` | Create field permission |
| GET | `/field-permissions` | Search/filter field permissions |
| GET | `/field-permissions/{id}` | Get single field permission |
| PATCH | `/field-permissions/{id}` | Update allowed fields |
| DELETE | `/field-permissions/{id}` | Delete field permission |

**Required permission:** `AUTH:FIELD_PERMISSION` (CREATE/READ/UPDATE/DELETE)

## Creating Field Permissions

### Request Body

```json
{
  "role_id": "<uuid>",
  "resource": "SERVICE_KEY:ENTITY_KEY",
  "action": 1,
  "fields": ["field1", "field2", "field3"]
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_id` | UUID | Yes | The role to configure |
| `resource` | String | Yes | Resource in `SERVICE_KEY:ENTITY_KEY` format (UPPER_SNAKE_CASE) |
| `action` | Integer | Yes | `1` = READ, `4` = UPDATE |
| `fields` | String[] | Yes | Allowed field names (`id` is always auto-included, no need to list it) |

### Resource Format

Same format as existing permissions:
```
SERVICE_KEY:ENTITY_KEY
```

Examples:
- `PROFILE:PROFILE`
- `LOOKUP:TYPE`
- `LOOKUP:ITEM`
- `WALLET:TRANSACTION`
- `AUTH:USER`

### Action Values

| Value | Constant | Meaning |
|-------|----------|---------|
| `1` | READ | Controls which fields appear in GET responses |
| `4` | UPDATE | Controls which fields can be sent in PATCH requests |

## Examples

### Example 1: Allow SUPPORT_AGENT to read limited profile fields

```http
POST /field-permissions
Content-Type: application/json
baggage: accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000

{
  "role_id": "550e8400-e29b-41d4-a716-446655440001",
  "resource": "PROFILE:PROFILE",
  "action": 1,
  "fields": ["name", "email", "phone", "created_at"]
}
```

**Result:** When a user with role SUPPORT_AGENT calls `GET /profiles/123`, they receive:
```json
{
  "id": "...",
  "name": "John Doe",
  "email": "john@example.com",
  "phone": "+1-555-0100",
  "created_at": "2024-01-15T10:30:00"
}
```
Fields like `address`, `ssn`, `preferences` are omitted.

### Example 2: Allow SUPPORT_AGENT to update only name and email

```http
POST /field-permissions
Content-Type: application/json
baggage: accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000

{
  "role_id": "550e8400-e29b-41d4-a716-446655440001",
  "resource": "PROFILE:PROFILE",
  "action": 4,
  "fields": ["name", "email"]
}
```

**Result:** When the user tries `PATCH /profiles/123 {"phone": "+1234"}`, they get:
```json
{
  "error": "FIELD_NOT_PERMITTED",
  "message": "Field 'phone' is not permitted for update",
  "field": "phone"
}
```

### Example 3: Configure lookup type fields for VIEWER role

```http
POST /field-permissions
{
  "role_id": "<VIEWER_role_id>",
  "resource": "LOOKUP:TYPE",
  "action": 1,
  "fields": ["code", "name", "description", "is_active"]
}
```

### Example 4: Configure nested relation fields

When a response includes nested entities (e.g., `items` inside a lookup type), configure field permissions for the nested entity's resource separately:

```http
POST /field-permissions
{
  "role_id": "<VIEWER_role_id>",
  "resource": "LOOKUP:ITEM",
  "action": 1,
  "fields": ["code", "value", "is_active"]
}
```

**Result:** `GET /lookup-types/123?includes=items` returns:
```json
{
  "id": "...",
  "code": "COUNTRY",
  "name": "Countries",
  "items": [
    { "id": "...", "code": "US", "value": "United States", "is_active": true },
    { "id": "...", "code": "UK", "value": "United Kingdom", "is_active": true }
  ]
}
```

## Updating Field Permissions

Change the allowed fields for an existing entry:

```http
PATCH /field-permissions/{id}
Content-Type: application/json
baggage: accesses=ADMIN_ALL*,user_id=...,client_id=...

{
  "fields": ["name", "email", "phone", "address"]
}
```

## Querying Field Permissions

### By resource (service-scoped)

```http
GET /field-permissions?resource=sw|LOOKUP:
```

### By role

```http
GET /field-permissions?role_id=eq|550e8400-e29b-41d4-a716-446655440001
```

### By action

```http
GET /field-permissions?action=eq|1
```

### Combined filters

```http
GET /field-permissions?resource=sw|PROFILE:&action=eq|1&page=1&page_size=20
```

## Behavior Rules

| Scenario | Result |
|----------|--------|
| No `field_permissions` entry exists for the resource | **All fields visible** (migration mode — backward compatible) |
| Entry exists with `["name", "email"]` for READ | GET returns only `id`, `name`, `email` |
| Entry exists with `["name"]` for UPDATE | PATCH rejects any field other than `name` |
| User has multiple roles with entries | **UNION** of all roles' allowed fields |
| User has role `ADMIN_ALL` | All fields visible, no filtering applied |
| `PublicAccess` endpoint | No field filtering (public DTOs are limited by design) |
| `id` field | **Always included** in responses, never needs to be listed |

## Constraints

- **Unique:** One entry per `(role_id, resource, action)` combination
- **Cascade delete:** Deleting a role removes all its field permissions
- **Resource format:** Must be `UPPER_SNAKE_CASE:UPPER_SNAKE_CASE` with `:` separator
- **Action values:** Only `1` (READ) and `4` (UPDATE) are valid

## Migration Strategy

1. **Deploy** the auth service with the new `field_permissions` table
2. **Initially** — no entries exist, so all services behave as before (all fields visible)
3. **Gradually configure** field permissions per role as needed
4. Once **any role** has a field_permission entry for a resource, the deny-all default activates for that resource
5. Ensure all roles that access that resource have appropriate field permissions configured

## Sync Mechanism

Each service syncs field permissions from the auth service every 30 seconds (same as resource-level permissions). Changes take effect within 30 seconds of creation/update.

Environment variables:
- `AUTH_FIELD_PERMISSIONS_ENDPOINT` — URL to the field-permissions endpoint (optional, defaults to deriving from `AUTH_ROLES_ENDPOINT`)
- `AUTH_ROLES_BAGGAGE_HEADER` — Baggage header for service-to-service auth (existing)
