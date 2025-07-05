## OAuth2 Authentication Server: Database Schema

Here is a comprehensive database schema designed to support a full-featured OAuth2 authentication server. It covers users, client applications, and the various tokens and codes involved in the OAuth2 flows.

### Core Concepts

The schema is built around these key OAuth2 entities:

1.  **Users**: The individuals who own the resources. They log in to your server to grant or deny access.
2.  **Clients**: The third-party applications (e.g., a mobile app, a web app) that want to access a user's resources.
3.  **Authorization Codes**: Short-lived, single-use codes issued when a user grants consent. The client exchanges this code for an access token.
4.  **Access Tokens**: The keys the client uses to access the user's resources on the resource server. These are typically short-lived.
5.  **Refresh Tokens**: Long-lived tokens used to obtain new access tokens without requiring the user to log in again.
6.  **Scopes**: Define the specific permissions the client is requesting (e.g., `read:profile`, `write:calendar`).

---

### Database Table Design

Below are the detailed schemas for each table.

#### 1. `users`
This table stores the primary identity of your users.

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `id` | UUID / INT | **Primary Key**. UUID is recommended over auto-incrementing integers. |
| `username` | VARCHAR(255) | **Unique**. Used for login. |
| `email` | VARCHAR(255) | **Unique**. Used for communication and can be used for login. |
| `password_hash` | VARCHAR(255) | **Required**. **NEVER** store plain-text passwords. Use a strong hashing algorithm like Argon2 or bcrypt. |
| `is_active` | BOOLEAN | Defaults to `true`. Allows for deactivating user accounts. |
| `created_at` | TIMESTAMP | Automatically set to the time of creation. |
| `updated_at` | TIMESTAMP | Automatically updated on any change to the record. |

#### 2. `clients`
This table holds information about the applications that are registered to use your authentication service.

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `client_id` | VARCHAR(255) | **Primary Key**. A unique, randomly generated string. This is public. |
| `client_secret` | VARCHAR(255) | **Required**. A secret key for the client. Should also be **hashed** in the database. |
| `name` | VARCHAR(255) | **Required**. A human-readable name for the application (e.g., "My Awesome Photo App"). |
| `redirect_uris` | TEXT / JSON | **Required**. A list of allowed URLs to redirect users to after authorization. Storing as a JSON array is ideal. |
| `allowed_grants`| TEXT / JSON | A list of OAuth2 grant types this client is permitted to use (e.g., `["authorization_code", "refresh_token"]`). |
| `created_at` | TIMESTAMP | Automatically set to the time of creation. |
| `updated_at` | TIMESTAMP | Automatically updated on any change to the record. |

#### 3. `auth_codes`
This table stores the temporary authorization codes. Records in this table should have a very short lifespan (e.g., 1-10 minutes).

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `code` | VARCHAR(255) | **Primary Key**. The unique, secure, random authorization code string. |
| `user_id` | UUID / INT | **Foreign Key** to `users.id`. The user who granted consent. |
| `client_id` | VARCHAR(255) | **Foreign Key** to `clients.client_id`. The client this code was issued to. |
| `scopes` | TEXT / JSON | The specific scopes that were approved by the user for this code. |
| `redirect_uri` | VARCHAR(2048) | The exact redirect URI used in the initial request. |
| `expires_at` | TIMESTAMP | **Required**. The timestamp when this code expires and can no longer be used. |

#### 4. `tokens`
This table stores the access and refresh tokens. It's the heart of the post-authorization flow.

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `id` | UUID / INT | **Primary Key**. |
| `access_token` | TEXT | **Unique, Indexed**. The access token string. Can be a JWT or an opaque string. |
| `refresh_token` | TEXT | **Unique, Indexed**. The refresh token string. |
| `user_id` | UUID / INT | **Foreign Key** to `users.id`. The user this token belongs to. |
| `client_id` | VARCHAR(255) | **Foreign Key** to `clients.client_id`. The client using this token. |
| `scopes` | TEXT / JSON | The scopes associated with this token. |
| `access_token_expires_at` | TIMESTAMP | The expiration timestamp for the `access_token`. |
| `refresh_token_expires_at`| TIMESTAMP | The expiration timestamp for the `refresh_token`. |
| `revoked_at` | TIMESTAMP | `NULL` by default. Set to the current time if the token is revoked before expiry. |
| `created_at` | TIMESTAMP | Automatically set to the time of creation. |


#### 5. `scopes`
This table is the master list of all possible permissions your API supports.

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `id` | UUID / INT | **Primary Key**. |
| `name` | VARCHAR(255) | **Unique, Indexed**. The machine-readable name of the scope (e.g., read:calendar). |
| `description` | TEXT | **Required**. A human-readable description shown to users on the consent screen. |
| `created_at` | TIMESTAMP | Automatically set to the time of creation. |
| `updated_at` | TIMESTAMP | Automatically set to the time of creation. |


#### 6. `client_scopes`
This table is the master list of all possible permissions your API supports.

| Column Name | Data Type | Constraints & Notes |
| :--- | :--- | :--- |
| `id` | UUID / INT | **Primary Key**. |
| `client_id` | VARCHAR(255) | **Unique, Indexed**. Composite Primary Key, Foreign Key to clients.client_id. |
| `scope_id` | UUID | **Required**. Composite Primary Key, Foreign Key to scopes.id. |
| `created_at` | TIMESTAMP | Automatically set to the time of creation. |
| `updated_at` | TIMESTAMP | Automatically set to the time of creation. |

---

### Entity-Relationship Diagram (ERD) Overview

The relationships between the tables are as follows:

* A `user` can have many `auth_codes` and many `tokens`.
* A `client` can have many `auth_codes` and many `tokens`.
* An `auth_code` belongs to one `user` and one `client`.
* A `token` belongs to one `user` and one `client`.

This structure ensures that every token and code is tightly bound to the user who authorized it and the client it was issued for.

### Example SQL (PostgreSQL)

Here is a sample SQL script to create these tables.

```sql
-- For using UUIDs as primary keys
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Clients Table
CREATE TABLE clients (
    client_id VARCHAR(255) PRIMARY KEY,
    client_secret VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    redirect_uris JSONB NOT NULL, -- Using JSONB for efficient querying of URIs
    allowed_grants JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Authorization Codes Table
CREATE TABLE auth_codes (
    code VARCHAR(255) PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id VARCHAR(255) NOT NULL REFERENCES clients(client_id) ON DELETE CASCADE,
    scopes JSONB,
    redirect_uri VARCHAR(2048) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

-- Index for faster cleanup of expired codes
CREATE INDEX idx_auth_codes_expires_at ON auth_codes(expires_at);

-- 4. Tokens Table
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    access_token TEXT NOT NULL UNIQUE,
    refresh_token TEXT NOT NULL UNIQUE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id VARCHAR(255) NOT NULL REFERENCES clients(client_id) ON DELETE CASCADE,
    scopes JSONB,
    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for fast token lookups
CREATE INDEX idx_tokens_access_token ON tokens(access_token);
CREATE INDEX idx_tokens_refresh_token ON tokens(refresh_token);
CREATE INDEX idx_tokens_user_id ON tokens(user_id);


-- 5. Scopes Table (Master List)
CREATE TABLE scopes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create an index on the scope name for fast lookups
CREATE INDEX idx_scopes_name ON scopes(name);


-- 6. Client Scopes Join Table (Permissions)
CREATE TABLE client_scopes (
    client_id VARCHAR(255) NOT NULL REFERENCES clients(client_id) ON DELETE CASCADE,
    scope_id UUID NOT NULL REFERENCES scopes(id) ON DELETE CASCADE,
    -- Define a composite primary key to ensure each client-scope pair is unique.
    PRIMARY KEY (client_id, scope_id)
);
