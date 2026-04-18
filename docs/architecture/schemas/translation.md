# Translation Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    projects {
        uuid id PK
        uuid user_id
        varchar name
        varchar api_key UNIQUE
        varchar default_locale
        timestamp created_at
        timestamp updated_at
    }
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    tags {
        uuid id PK
        varchar name UNIQUE
        timestamp created_at
        timestamp updated_at
    }
    translation_keys {
        uuid id PK
        uuid project_id FK
        uuid user_id
        varchar key_name
        text description
        timestamp created_at
        timestamp updated_at
    }
    translation_versions {
        uuid id PK
        uuid key_id FK
        varchar locale
        text content
        integer version_number
        varchar status
        uuid created_by
        timestamp created_at
        timestamp updated_at
    }
    key_tags {
        uuid key_id PK, FK
        uuid tag_id PK, FK
    }
    projects ||--o{ translation_keys : "has keys"
    translation_keys ||--o{ translation_versions : "has versions"
    translation_keys ||--o{ key_tags : "has tags"
    tags ||--o{ key_tags : "is tagged"
```

## Database Schema (translation)

### Tables

#### projects
| Column         | Type      | Default           | Constraints         |
|---------------|-----------|-------------------|---------------------|
| id            | uuid      | gen_random_uuid() | PK, NOT NULL        |
| user_id       | uuid      |                   | NOT NULL            |
| name          | varchar(255)|                  | NOT NULL            |
| api_key       | varchar(64)|                   | UNIQUE, NOT NULL    |
| default_locale| varchar(10)| 'en'              |                     |
| created_at    | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at    | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### tags
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL        |
| name        | varchar(50)|                   | UNIQUE, NOT NULL    |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### translation_keys
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL        |
| project_id  | uuid      |                   | NOT NULL, FK        |
| user_id     | uuid      |                   | NOT NULL            |
| key_name    | varchar(255)|                  | NOT NULL            |
| description | text      |                   |                     |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### translation_versions
| Column         | Type      | Default           | Constraints         |
|----------------|-----------|-------------------|---------------------|
| id             | uuid      | gen_random_uuid() | PK, NOT NULL        |
| key_id         | uuid      |                   | NOT NULL, FK        |
| locale         | varchar(10)|                   | NOT NULL            |
| content        | text      |                   | NOT NULL            |
| version_number | integer   |                   | NOT NULL            |
| status         | varchar(20)| 'draft'           | NOT NULL            |
| created_by     | uuid      |                   | NOT NULL            |
| created_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### key_tags
| Column   | Type | Default           | Constraints         |
|----------|------|-------------------|---------------------|
| key_id   | uuid |                   | PK, NOT NULL, FK    |
| tag_id   | uuid |                   | PK, NOT NULL, FK    |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |
