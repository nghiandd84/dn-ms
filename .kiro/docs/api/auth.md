# Auth Service API

Provides authentication, authorization, and identity management for the platform. Supports OAuth2 flows, role-based access control (RBAC), and user/client management.

## Main Responsibilities
- User registration, login, and credential management
- Token issuance, verification, and refresh (JWT, OAuth2 authorization code flow)
- Role, permission, and scope management (RBAC)
- OAuth2 client and authorization code management
- Public authentication request endpoints (code, login, register)
- User CRUD
- Integration with notification and profile services for onboarding and alerts

## Base URL
`http://localhost:5011`

---

## Key API Groups & Endpoints

### Authentication Requests — Public (`/public/requests`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/public/requests/code` | `AuthenticationCreateRequest` | `OkUuidResponse` |
| POST | `/public/requests/login` | `AuthLoginRequest` | `AuthLoginDataResponse` |
| POST | `/public/requests/register` | `AuthRegisterRequest` | `AuthRegisterDataResponse` |

### Login & Registration

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/login` | `LoginRequest` | `LoginDataResponse` |
| POST | `/register` | `UserForCreateRequest` | `OkUuidResponse` |

### Token Management (`/tokens`, `/public/tokens`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/public/tokens/oauth` | `TokenForCreateRequest` | `AuthorizationCodeDataResponse` |
| POST | `/public/tokens/verify` | `TokenForVerifyRequest` | `AccessTokenStructResponse` |
| GET | `/tokens` | — | `QueryResultResponse<TokenData>` |
| GET | `/tokens/{token_id}` | — | `TokenDataResponse` |

### User Management (`/users`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| GET | `/users` | — | `QueryResultResponse<UserData>` |
| GET | `/users/{user_id}` | — | `UserDataResponse` |
| DELETE | `/users/{user_id}` | — | `OkUuidResponse` |
| GET | `/test_users` | — | `QueryResultResponse<UserData>` |

### Role Management (`/roles`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/roles` | `RoleForCreateRequest` | `OkUuidResponse` |
| GET | `/roles` | — | `QueryResultResponse<RoleData>` |
| GET | `/roles/{role_id}` | — | `RoleDataResponse` |
| PATCH | `/roles/{role_id}` | `RoleForUpdateRequest` | `OkUuidResponse` |
| DELETE | `/roles/{role_id}` | — | `OkUuidResponse` |
| GET | `/roles/{role_id}/permissions` | — | `QueryResultResponse<PermissionData>` |
| POST | `/roles/{role_id}/assign-permissions` | `AssignPermissionToRoleRequest` | `OkUuidResponse` |
| POST | `/roles/{role_id}/unassign-permissions` | `AssignPermissionToRoleRequest` | `OkUuidResponse` |

Roles support `?includes=permissions` on GET endpoints to eager-load associated permissions.

### Permission Management (`/permissions`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/permissions` | `PermissionForCreateRequest` | `OkUuidResponse` |
| GET | `/permissions` | — | `QueryResultResponse<PermissionData>` |
| GET | `/permissions/{permission_id}` | — | `PermissionDataResponse` |
| PATCH | `/permissions/{permission_id}` | `PermissionForUpdateRequest` | `OkUuidResponse` |
| DELETE | `/permissions/{permission_id}` | — | `OkUuidResponse` |

### Scope Management (`/scopes`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/scopes` | `ScopeForCreateRequest` | `OkUuidResponse` |
| GET | `/scopes` | — | `QueryResultResponse<ScopeData>` |
| GET | `/scopes/{scope_id}` | — | `ScopeDataResponse` |
| PATCH | `/scopes/{scope_id}` | `ScopeForUpdateRequest` | `OkUuidResponse` |
| DELETE | `/scopes/{scope_id}` | — | `OkUuidResponse` |

### Client Management (`/clients`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/clients` | `ClientForCreateRequest` | `OkUuidResponse` |
| GET | `/clients` | — | `QueryResultResponse<ClientData>` |
| GET | `/clients/{client_id}` | — | `ClientDataResponse` |
| PATCH | `/clients/{client_id}` | `ClientForUpdateRequest` | `OkUuidResponse` |
| DELETE | `/clients/{client_id}` | — | `OkUuidResponse` |

### Auth Code Management (`/auth-codes`)

| Method | Path | Request Body | Response |
|--------|------|-------------|----------|
| POST | `/auth-codes` | `AuthCodeForCreateRequest` | `OkUuidResponse` |
| GET | `/auth-codes` | — | `QueryResultResponse<AuthCodeData>` |
| GET | `/auth-codes/{auth_code_id}` | — | `AuthCodeDataResponse` |
| DELETE | `/auth-codes/{auth_code_id}` | — | `OkUuidResponse` |

### Health Check

| Method | Path | Response |
|--------|------|----------|
| GET | `/healthchecker` | Health status |

---

## Data Models

### Response Models

#### UserData
| Field    | Type   |
|----------|--------|
| id       | UUID   |
| email    | String |
| age      | u32    |
| language | String |

#### RoleData
| Field       | Type                     | Notes |
|-------------|--------------------------|-------|
| id          | UUID                     |       |
| name        | String                   |       |
| description | String                   |       |
| client_id   | UUID                     |       |
| is_default  | bool                     |       |
| permissions | Vec\<PermissionData\>    | Only when `?includes=permissions` |

#### PermissionData
| Field       | Type   |
|-------------|--------|
| id          | UUID   |
| resource    | String |
| description | String |
| mask        | i32    |

#### ScopeData
| Field       | Type   |
|-------------|--------|
| id          | UUID   |
| name        | String |
| description | String |

#### ClientData
| Field          | Type         |
|----------------|--------------|
| id             | UUID         |
| client_secret  | String       |
| client_key     | String       |
| name           | String       |
| email          | String       |
| description    | String       |
| redirect_uris  | Vec\<String\> |
| allowed_grants | Vec\<String\> |

#### TokenData
| Field                     | Type         |
|---------------------------|--------------|
| id                        | UUID         |
| access_token              | String       |
| refresh_token             | String       |
| user_id                   | UUID         |
| client_id                 | UUID         |
| scopes                    | Vec\<String\> |
| access_token_expires_at   | DateTime     |
| refresh_token_expires_at  | DateTime     |
| revoked_at                | DateTime?    |
| created_at                | DateTime     |
| updated_at                | DateTime     |

#### AuthCodeData
| Field        | Type         |
|--------------|--------------|
| id           | UUID         |
| code         | String       |
| user_id      | UUID         |
| client_id    | UUID         |
| scopes       | Vec\<String\> |
| redirect_uri | String       |
| expires_at   | DateTime     |
| created_at   | DateTime     |
| updated_at   | DateTime     |

#### LoginData
| Field | Type   |
|-------|--------|
| code  | String |

#### AuthLoginData
| Field        | Type   |
|--------------|--------|
| id_token     | String |
| redirect_uri | String |

#### AuthRegisterData
| Field        | Type   |
|--------------|--------|
| id_token     | String |
| redirect_uri | String |

### Request Models

#### UserForCreateRequest (Register)
| Field    | Type   | Validation              |
|----------|--------|-------------------------|
| email    | String | 6–256 chars             |
| password | String | 10–128 chars            |
| language | String | 2–5 chars               |

#### LoginRequest
| Field        | Type         | Validation         |
|--------------|--------------|--------------------|
| client_id    | UUID         | required           |
| email        | String       | valid email        |
| password     | String       | 10–128 chars       |
| scopes       | Vec\<String\> | min 1 item         |
| redirect_uri | String       | valid URL          |

#### AuthenticationCreateRequest
| Field         | Type         | Validation     |
|---------------|--------------|----------------|
| client_id     | UUID         | required       |
| scopes        | Vec\<String\> | required, min 1 |
| redirect_uri  | String       | 1–1204 chars   |
| response_type | String       | 1–1204 chars   |
| state         | String       | 1–1204 chars   |

#### AuthLoginRequest
| Field    | Type   | Validation |
|----------|--------|------------|
| email    | String | required   |
| password | String | required   |
| state    | String | required   |

#### AuthRegisterRequest
| Field    | Type   | Validation |
|----------|--------|------------|
| email    | String | required   |
| password | String | required   |
| state    | String | required   |
| language | String | required   |

#### TokenForCreateRequest
| Field      | Type      | Validation |
|------------|-----------|------------|
| code       | String    | required   |
| client_id  | UUID      | required   |
| grant_type | GrantType | required (`authorization_code`, `refresh_token`, `client_credentials`) |

#### TokenForVerifyRequest
| Field | Type   | Validation |
|-------|--------|------------|
| token | String | required   |

#### RoleForCreateRequest
| Field       | Type   | Validation       |
|-------------|--------|------------------|
| name        | String | 1–50 chars       |
| description | String | 1–250 chars      |
| client_id   | UUID   | required         |
| is_default  | bool?  | optional         |

#### RoleForUpdateRequest
| Field       | Type   | Validation       |
|-------------|--------|------------------|
| name        | String | 1–50 chars       |
| description | String | 1–250 chars      |
| client_id   | UUID   | required         |
| is_default  | bool?  | optional         |

#### AssignPermissionToRoleRequest
| Field          | Type       |
|----------------|------------|
| permission_ids | Vec\<UUID\> |

#### PermissionForCreateRequest
| Field       | Type    | Validation    |
|-------------|---------|---------------|
| resource    | String  | 5–1024 chars  |
| description | String? | optional      |
| mask        | i32?    | optional (default 0) |

#### PermissionForUpdateRequest
| Field       | Type    | Validation    |
|-------------|---------|---------------|
| resource    | String  | 5–1024 chars  |
| description | String? | optional      |
| mask        | i32?    | optional      |

#### ScopeForCreateRequest
| Field       | Type    | Validation    |
|-------------|---------|---------------|
| name        | String  | 2–128 chars   |
| description | String? | 0–512 chars   |

#### ScopeForUpdateRequest
| Field       | Type    | Validation    |
|-------------|---------|---------------|
| description | String? | 0–512 chars   |

#### ClientForCreateRequest
| Field          | Type         | Validation         |
|----------------|--------------|--------------------|
| client_secret  | String       | 10–128 chars       |
| name           | String       | 2–128 chars        |
| client_key     | String?      | 0–512 chars        |
| email          | String?      | valid email        |
| description    | String?      | 0–512 chars        |
| redirect_uris  | Vec\<String\> | min 1 item         |
| allowed_grants | Vec\<String\> | min 1 item         |

#### ClientForUpdateRequest
Same fields as create, all optional.

#### AuthCodeForCreateRequest
| Field        | Type         | Validation         |
|--------------|--------------|--------------------|
| user_id      | UUID         | required           |
| client_id    | UUID         | required           |
| scopes       | Vec\<String\> | required, min 1    |
| redirect_uri | String       | 1–1204 chars       |

---

## Query Parameters

All list endpoints support pagination, ordering, and column-based filtering.

| Param           | Description                                      | Example                              |
|-----------------|--------------------------------------------------|--------------------------------------|
| page            | Page number                                      | `?page=1`                            |
| page_size       | Items per page                                   | `?page_size=20`                      |
| order_name      | Column to order by                               | `?order_name=name`                   |
| order_direction | 0 = ASC, 1 = DESC                               | `?order_direction=1`                 |
| includes        | Eager-load related entities (roles only)         | `?includes=permissions`              |
| {column}        | Filter by column with operator prefix            | `?resource=sw\|AUTH`, `?email=eq\|test@test.com` |

### Filter Operators
| Operator | Meaning       | Example                                    |
|----------|---------------|--------------------------------------------|
| `eq\|`   | Equal         | `?email=eq\|test@test.com`                 |
| `li\|`   | Like          | `?resource=li\|AUTH`                       |
| `sw\|`   | Starts with   | `?resource=sw\|AUTH`                       |
| `in\|`   | In list       | `?id=in\|uuid1,uuid2`                     |
| `nin\|`  | Not in list   | `?allowed_grants=nin\|auth_code`           |
| `lte\|`  | Less or equal | `?age=lte\|12`                             |

---

## Entity Relations

```
User ←→ Role        (many-to-many via `access` table)
Role ←→ Permission  (many-to-many via `role_permissions` table)
Role → Client       (belongs_to via `client_id`)
Token → User        (via `user_id`)
Token → Client      (via `client_id`)
AuthCode → User     (via `user_id`)
AuthCode → Client   (via `client_id`)
```

### Database Tables
| Table             | Primary Key | Notable Columns |
|-------------------|-------------|-----------------|
| users             | UUID        | email (unique), password, language, confirmed, two_factor_enabled, is_active |
| roles             | UUID        | name (unique), description, client_id, is_default |
| permissions       | UUID        | resource (unique), mask, description |
| role_permissions  | UUID        | role_id, permission_id (junction table) |
| access            | UUID        | user_id, role_id (junction table) |
| clients           | UUID        | client_secret, client_key, name, email, redirect_uris, allowed_grants |
| scopes            | UUID        | name (unique), description |
| tokens            | UUID        | access_token, refresh_token, user_id, client_id, scopes, expires_at |
| auth_codes        | UUID        | code (unique, auto-generated 64-char), user_id, client_id, scopes, expires_at (1 min) |

---

## Architecture Layers

```
API (apis/auth/src/routes/*.rs)
  → Service (features/auth/service/src/*.rs)
    → Repo Query/Mutation (features/auth/repo/src/*/)
      → Entity (features/auth/entities/src/*.rs)
```

- **Entity**: SeaORM models with `before_save` hooks for auto-timestamps. `Dto` macro generates `ForCreateDto`, `ForUpdateDto`, and `ModelOptionDto`.
- **Model**: Request DTOs with validation (`ValidJson`), response DTOs with `Response` + `ParamFilter` derives for auto-generated filter params.
- **Repo**: `*Query` structs use `#[derive(Query)]` macro for filtered/paginated queries. `*Mutation` structs use `#[derive(Mutation)]` macro for create/update/delete. Role queries support `query_related` for eager-loading permissions.
- **Service**: Business logic layer — handles password hashing, token generation, role assignment, OAuth2 flows. Delegates persistence to repo.
- **API Routes**: Axum handlers with `ValidJson` for validated input, `Query<Pagination>`, `Query<Order>`, `Query<FilterParams>` for list queries. All registered under `Router` with `AppState<AuthAppState, AuthCacheState>`.

## Security & Patterns
- JWT for stateless authentication
- OAuth2 authorization code flow with PKCE support
- RBAC: Users → Roles → Permissions
- Public endpoints prefixed with `/public/` (no auth required)
- Password hashing via argon2
- Auth codes expire after 1 minute
- All endpoints documented with OpenAPI (Swagger UI at `/swagger-ui/`)

## Integration Points
- **Profile Service**: Links user profiles on registration/login
- **Notification Service**: Sends onboarding, verification, and security alerts via Kafka streams
- **Other Services**: Issues tokens and permissions consumed by all microservices; permission sync via Consul

## Example Usage

```http
### Register a new user
POST /register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "Test@123456",
  "language": "en"
}

### Login
POST /login
Content-Type: application/json

{
  "client_id": "5ed8e536-12ae-463d-ae9b-8c78cc5481e1",
  "email": "user@example.com",
  "password": "Test@123456",
  "scopes": ["email", "profile"],
  "redirect_uri": "http://localhost:4200"
}

### Exchange auth code for token
POST /public/tokens/oauth
Content-Type: application/json

{
  "client_id": "5ed8e536-12ae-463d-ae9b-8c78cc5481e1",
  "code": "<auth_code>",
  "grant_type": "authorization_code"
}

### List roles with permissions
GET /roles?includes=permissions&page=1&page_size=10

### Create a permission
POST /permissions
Content-Type: application/json

{
  "resource": "AUTH_MANAGE_USERS",
  "description": "Manage users",
  "mask": 1
}

### Assign permissions to a role
POST /roles/{role_id}/assign-permissions
Content-Type: application/json

{
  "permission_ids": ["<permission_uuid>"]
}

### Filter permissions by resource
GET /permissions?resource=sw|AUTH&page=1&page_size=20
```
