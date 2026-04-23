# Payment Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    payment_attempts {
        uuid id PK
        uuid payment_id FK
        varchar provider
        json raw_response
        boolean success
        text error_message
        timestamp created_at
    }
    payment_method_limits {
        uuid id PK
        uuid payment_method_id FK
        varchar currency
        bigint min_amount
        bigint max_amount
        timestamp created_at
        timestamp updated_at
    }
    payment_methods {
        uuid id PK
        varchar display_name
        varchar provider_name
        json provider_config
        varchar[] supported_countries
        varchar[] supported_currencies
        integer priority
        boolean is_active
        real fee_percentage
        text icon_url
        timestamp created_at
        timestamp updated_at
    }
    payments {
        uuid id PK
        varchar transaction_id UNIQUE
        uuid user_id
        bigint amount
        varchar currency
        varchar status
        varchar provider_name
        varchar gateway_transaction_id UNIQUE
        varchar idempotency_key UNIQUE
        timestamp created_at
        timestamp updated_at
    }
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    payments ||--o{ payment_attempts : "has attempts"
    payment_methods ||--o{ payment_method_limits : "has limits"
```

## Database Schema (payment)

### Tables

#### payments
| Column                | Type      | Default                | Constraints         |
|-----------------------|-----------|------------------------|---------------------|
| id                    | uuid      | gen_random_uuid()      | PK, NOT NULL        |
| transaction_id        | varchar(255)|                       | UNIQUE, NOT NULL    |
| user_id               | uuid      |                        | NOT NULL            |
| amount                | bigint    |                        | NOT NULL            |
| currency              | varchar(3)|                        | NOT NULL            |
| status                | varchar(50)| 'created'              | NOT NULL            |
| provider_name         | varchar(50)|                        |                     |
| gateway_transaction_id| varchar(255)|                       | UNIQUE              |
| idempotency_key       | varchar(255)|                       | UNIQUE              |
| created_at            | timestamp | CURRENT_TIMESTAMP      | NOT NULL            |
| updated_at            | timestamp | CURRENT_TIMESTAMP      | NOT NULL            |

---

#### payment_attempts
| Column         | Type      | Default           | Constraints         |
|----------------|-----------|-------------------|---------------------|
| id             | uuid      | gen_random_uuid() | PK, NOT NULL        |
| payment_id     | uuid      |                   | NOT NULL, FK        |
| provider       | varchar(50)|                   | NOT NULL            |
| raw_response   | json      |                   | NOT NULL            |
| success        | boolean   | false             | NOT NULL            |
| error_message  | text      |                   |                     |
| created_at     | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### payment_methods
| Column              | Type      | Default           | Constraints         |
|---------------------|-----------|-------------------|---------------------|
| id                  | uuid      | gen_random_uuid() | PK, NOT NULL        |
| display_name        | varchar(100)|                  | NOT NULL            |
| provider_name       | varchar(50)|                   | NOT NULL            |
| provider_config     | json      |                   | NOT NULL            |
| supported_countries | varchar[] |                   | NOT NULL            |
| supported_currencies| varchar[] |                   | NOT NULL            |
| priority            | integer   | 1                 | NOT NULL            |
| is_active           | boolean   | true              | NOT NULL            |
| fee_percentage      | real      |                   |                     |
| icon_url            | text      |                   |                     |
| created_at          | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at          | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### payment_method_limits
| Column            | Type      | Default           | Constraints         |
|-------------------|-----------|-------------------|---------------------|
| id                | uuid      | gen_random_uuid() | PK, NOT NULL        |
| payment_method_id | uuid      |                   | NOT NULL, FK        |
| currency          | varchar(3)|                   | NOT NULL            |
| min_amount        | bigint    | 0                 | NOT NULL            |
| max_amount        | bigint    |                   |                     |
| created_at        | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at        | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

