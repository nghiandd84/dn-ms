
# Auth Service

The Auth Service provides comprehensive authentication, authorization, and identity management for the platform. It exposes a rich set of RESTful APIs and supports OAuth2 flows, role-based access control, and integration with other services.

## Main Responsibilities

# Auth Service

Handles authentication, authorization, and user identity management.
- Issues and validates JWT tokens
- Manages user credentials (registration, password reset, MFA)
- Enforces permissions and roles
- Integrates with profile and notification services for onboarding and alerts
- Typical endpoints: `/login`, `/register`, `/token/refresh`, `/me`, `/permissions`
	- `GET /users/{user_id}` — Get user details
	- `DELETE /users/{user_id}` — Delete user

- **Role & Permission Management**
	- `GET /roles`, `POST /roles`, `PATCH /roles/{role_id}` — Manage roles
	- `POST /roles/{role_id}/assign-permissions` — Assign permissions to role
	- `GET /roles/{role_id}/permissions` — List permissions for a role
	- `GET /permissions`, `POST /permissions`, `PATCH /permissions/{permission_id}` — Manage permissions

- **Scope Management**
	- `GET /scopes`, `POST /scopes`, `PATCH /scopes/{scope_id}` — Manage OAuth2 scopes

- **Client Management**
	- `GET /clients`, `POST /clients`, `PATCH /clients/{client_id}` — Manage OAuth2 clients

- **Auth Code Management**
	- `GET /auth-codes`, `POST /auth-codes`, `DELETE /auth-codes/{auth_code_id}` — Manage authorization codes

- **Authentication Requests**
	- `POST /public/requests/code` — Request authentication code (for MFA, passwordless, etc.)

- **Health Check**
	- `GET /healthchecker` — Service health endpoint

## Data Models
- `UserData`, `UserForCreateRequest` — User info and registration
- `LoginRequest`, `LoginData` — Login payloads and responses
- `AccessTokenStruct`, `TokenData` — JWT and OAuth2 tokens
- `RoleData`, `PermissionData`, `ScopeData` — RBAC entities
- `ClientData`, `AuthCodeData` — OAuth2 client and code management

## Integration Points
- **Profile Service**: Enriches user data and links profiles on registration/login
- **Notification Service**: Sends onboarding, verification, and security alerts
- **Other APIs**: Issues tokens and permissions for access to all microservices

## Security & Patterns
- Uses JWT for stateless authentication
- Supports OAuth2 flows (authorization code, PKCE)
- Implements RBAC with roles, permissions, and scopes
- All endpoints documented with OpenAPI (see `/swagger-ui/`)

## Example Usage
```http
POST /login
{
	"username": "user@example.com",
	"password": "password123"
}

Response:
{
	"access_token": "...",
	"user": { ... }
}
```
