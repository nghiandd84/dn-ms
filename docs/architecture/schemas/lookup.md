# Lookup Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    lookup_item_translations {
        uuid id PK
        uuid lookup_item_id FK
        varchar locale
        varchar name
        timestamp created_at
        timestamp updated_at
    }
    lookup_items {
        uuid id PK
        uuid lookup_type_id FK
        varchar code
        varchar name
        varchar url
        varchar query_param_one
        varchar query_param_two
        varchar[] tenants
        boolean is_active
        integer sort_order
        timestamp created_at
        timestamp updated_at
        json meta
    }
    lookup_types {
        uuid id PK
        varchar code UNIQUE
        varchar name
        text description
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    lookup_types ||--o{ lookup_items : "has items"
    lookup_items ||--o{ lookup_item_translations : "has translations"
```

## Database Schema (lookup)

### Tables

#### lookup_types
| Column        | Type      | Default           | Constraints         |
|--------------|-----------|-------------------|---------------------|
| id           | uuid      | gen_random_uuid() | PK, NOT NULL        |
| code         | varchar(50)|                   | UNIQUE, NOT NULL    |
| name         | varchar(100)|                  | NOT NULL            |
| description  | text      |                   |                     |
| is_active    | boolean   | true              | NOT NULL            |
| created_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at   | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### lookup_items
| Column           | Type        | Default           | Constraints         |
|------------------|-------------|-------------------|---------------------|
| id               | uuid        | gen_random_uuid() | PK, NOT NULL        |
| lookup_type_id   | uuid        |                   | NOT NULL, FK        |
| code             | varchar(50) |                   | NOT NULL            |
| name             | varchar(200)|                   | NOT NULL            |
| url              | varchar     | ''                | NOT NULL            |
| query_param_one  | varchar     | ''                | NOT NULL            |
| query_param_two  | varchar     | ''                | NOT NULL            |
| tenants          | varchar[]   | '{}'              | NOT NULL            |
| is_active        | boolean     | true              | NOT NULL            |
| sort_order       | integer     | 0                 | NOT NULL            |
| created_at       | timestamp   | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp   | CURRENT_TIMESTAMP | NOT NULL            |
| meta             | json        | '{}'              | NOT NULL            |

---

#### lookup_item_translations
| Column         | Type      | Default           | Constraints         |
|----------------|-----------|-------------------|---------------------|
| id             | uuid      | gen_random_uuid() | PK, NOT NULL        |
| lookup_item_id | uuid      |                   | NOT NULL, FK        |
| locale         | varchar(10)|                   | NOT NULL            |
| name           | varchar(200)|                  | NOT NULL            |
| created_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

