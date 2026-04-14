## Database Schema (auth)

### Tables

#### access
| Column      | Type      | Default                        | Constraints         |
|-------------|-----------|--------------------------------|---------------------|
| id          | uuid      | gen_random_uuid()              | PK                  |
| user_id     | uuid      |                                | NOT NULL            |
| role_id     | uuid      |                                | NOT NULL            |
| key         | varchar   | ''                             | NOT NULL            |
| created_at  | timestamp | CURRENT_TIMESTAMP              | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP              | NOT NULL            |

#### active_codes
| Column         | Type      | Default           | Constraints         |
|----------------|-----------|-------------------|---------------------|
| id             | uuid      | gen_random_uuid() | PK                  |
| code           | varchar(50)|                   | UNIQUE, NOT NULL    |
| is_used        | boolean   |                   | NOT NULL            |
| expiration_time| timestamp |                   | NOT NULL            |
| created_at     | timestamp |                   | NOT NULL            |
| updated_at     | timestamp |                   | NOT NULL            |
| user_id        | uuid      |                   | NOT NULL            |

#### auth_codes
| Column       | Type      | Default           | Constraints         |
|--------------|-----------|-------------------|---------------------|
| id           | uuid      | gen_random_uuid() | PK                  |
| code         | varchar   |                   | UNIQUE, NOT NULL    |
| client_id    | uuid      |                   | NOT NULL, FK        |
| user_id      | uuid      |                   | NOT NULL, FK        |
| redirect_uri | varchar   |                   | NOT NULL            |
| scopes       | varchar[] |                   | NOT NULL            |
| expires_at   | timestamp |                   | NOT NULL            |
| created_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

#### authentication_requests
| Column        | Type      | Default | Constraints         |
|---------------|-----------|---------|---------------------|
| id            | uuid      | gen_random_uuid() | PK          |
| client_id     | uuid      |         | NOT NULL, FK        |
| scopes        | varchar[] |         | NOT NULL            |
| response_type | varchar   |         | NOT NULL            |
| redirect_uri  | varchar   |         | NOT NULL            |
| state         | varchar   |         | NOT NULL            |
| expires_at    | timestamp |         | NOT NULL            |
| created_at    | timestamp |         | NOT NULL            |
| updated_at    | timestamp |         | NOT NULL            |

#### client_scopes
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK                  |
| client_id   | uuid      |                   | NOT NULL, FK        |
| scope_id    | uuid      |                   | NOT NULL, FK        |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

#### clients
| Column         | Type      | Default           | Constraints         |
|----------------|-----------|-------------------|---------------------|
| id             | uuid      | gen_random_uuid() | PK                  |
| client_secret  | varchar   |                   | NOT NULL            |
| name           | varchar(128)|                  | UNIQUE, NOT NULL    |
| description    | varchar(512)|                  |                     |
| redirect_uris  | varchar[] |                   | NOT NULL            |
| allowed_grants | varchar[] |                   | NOT NULL            |
| created_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| client_key     | varchar   |                   | NOT NULL            |
| email          | varchar   | ''                | NOT NULL            |

#### permissions
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK                  |
| resource    | varchar(1024)|                  | UNIQUE, NOT NULL    |
| description | varchar(250)|                   | NOT NULL            |
| mask        | integer   | 0                 |                     |
| created_at  | timestamp |                   | NOT NULL            |
| updated_at  | timestamp |                   | NOT NULL            |

#### role_permissions
| Column        | Type      | Default           | Constraints         |
|---------------|-----------|-------------------|---------------------|
| id            | uuid      | gen_random_uuid() | PK                  |
| role_id       | uuid      |                   | NOT NULL, FK        |
| permission_id | uuid      |                   | NOT NULL, FK        |
| created_at    | timestamp |                   | NOT NULL            |
| updated_at    | timestamp |                   | NOT NULL            |

#### roles
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK                  |
| name        | varchar(50)|                   | UNIQUE, NOT NULL    |
| description | varchar(250)|                   | NOT NULL            |
| created_at  | timestamp |                   | NOT NULL            |
| updated_at  | timestamp |                   | NOT NULL            |
| client_id   | uuid      | '00000000-0000-0000-0000-000000000000' | NOT NULL |
| is_default  | boolean   | false             | NOT NULL            |

#### scopes
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK                  |
| name        | varchar(128)|                  | UNIQUE, NOT NULL    |
| description | varchar(512)|                   |                     |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

#### tokens
| Column                  | Type      | Default           | Constraints         |
|-------------------------|-----------|-------------------|---------------------|
| id                      | uuid      | gen_random_uuid() | PK                  |
| access_token            | varchar   |                   | UNIQUE, NOT NULL    |
| refresh_token           | varchar   |                   | UNIQUE, NOT NULL    |
| user_id                 | uuid      |                   | NOT NULL, FK        |
| client_id               | uuid      |                   | NOT NULL, FK        |
| scopes                  | varchar[] |                   | NOT NULL            |
| access_token_expires_at | timestamp |                   | NOT NULL            |
| refresh_token_expires_at| timestamp |                   | NOT NULL            |
| revoked_at              | timestamp |                   |                     |
| created_at              | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at              | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

#### users
| Column              | Type      | Default           | Constraints         |
|---------------------|-----------|-------------------|---------------------|
| id                  | uuid      | gen_random_uuid() | PK                  |
| email               | varchar(250)|                  | UNIQUE, NOT NULL    |
| confirmed           | boolean   | false             | NOT NULL            |
| two_factor_enabled  | boolean   | false             | NOT NULL            |
| version             | smallint  | 1                 | NOT NULL            |
| password            | text      |                   | NOT NULL            |
| created_at          | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at          | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| is_active           | boolean   | false             | NOT NULL            |
| language            | varchar   | 'en-US'           | NOT NULL            |

### Indexes & Constraints

- PK = Primary Key
- FK = Foreign Key
- UNIQUE = Unique Constraint

### Foreign Keys (selected)

- auth_codes.client_id → clients.id (ON DELETE CASCADE)
- auth_codes.user_id → users.id (ON DELETE CASCADE)
- client_scopes.client_id → clients.id (ON DELETE CASCADE)
- client_scopes.scope_id → scopes.id (ON DELETE CASCADE)
- tokens.client_id → clients.id (ON DELETE CASCADE)
- tokens.user_id → users.id (ON DELETE CASCADE)
