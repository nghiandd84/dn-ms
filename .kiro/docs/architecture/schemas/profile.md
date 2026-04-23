# Profile Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    profiles {
        uuid id PK
        uuid user_id UNIQUE
        varchar first_name
        varchar last_name
        text bio
        varchar avatar_url
        varchar location
        timestamp created_at
        timestamp updated_at
    }
    social_links {
        uuid id PK
        uuid profile_id FK
        varchar platform
        varchar url
        timestamp created_at
        timestamp updated_at
    }
    user_preferences {
        uuid id PK
        uuid profile_id FK
        varchar language
        varchar theme
        boolean notifications_enabled
        timestamp created_at
        timestamp updated_at
    }
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    profiles ||--o{ social_links : "has links"
    profiles ||--o{ user_preferences : "has preferences"
```

## Database Schema (profile)

### Tables

#### profiles
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL        |
| user_id     | uuid      |                   | UNIQUE, NOT NULL    |
| first_name  | varchar(100)|                  | NOT NULL            |
| last_name   | varchar(100)|                  | NOT NULL            |
| bio         | text      |                   |                     |
| avatar_url  | varchar(500)|                  |                     |
| location    | varchar(255)|                  |                     |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### social_links
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL        |
| profile_id  | uuid      |                   | NOT NULL, FK        |
| platform    | varchar(50)|                   | NOT NULL            |
| url         | varchar(500)|                   | NOT NULL            |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### user_preferences
| Column                | Type      | Default           | Constraints         |
|-----------------------|-----------|-------------------|---------------------|
| id                    | uuid      | gen_random_uuid() | PK, NOT NULL        |
| profile_id            | uuid      |                   | NOT NULL, FK        |
| language              | varchar(10)|                   | NOT NULL            |
| theme                 | varchar(20)|                   | NOT NULL            |
| notifications_enabled | boolean   | true              | NOT NULL            |
| created_at            | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at            | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

