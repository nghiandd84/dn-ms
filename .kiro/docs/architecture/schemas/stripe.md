# Stripe Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    stripe_api_logs {
        uuid id PK
        varchar endpoint
        varchar method
        text request_body
        text response_body
        integer status_code
        text error_message
        varchar stripe_request_id
        timestamp created_at
        timestamp updated_at
    }
    stripe_payment_intents {
        uuid id PK
        uuid payment_id
        varchar stripe_payment_intent_id
        bigint amount
        varchar currency
        varchar status
        varchar client_secret
        json metadata
        timestamp created_at
        timestamp updated_at
    }
    stripe_refunds {
        uuid id PK
        uuid payment_id
        varchar stripe_refund_id UNIQUE
        varchar stripe_payment_intent_id
        bigint amount
        varchar currency
        varchar status
        varchar reason
        json metadata
        timestamp created_at
        timestamp updated_at
    }
    stripe_webhook_events {
        uuid id PK
        varchar stripe_event_id UNIQUE
        varchar event_type
        json event_data
        boolean processed
        text processing_error
        timestamp created_at
        timestamp updated_at
    }
```

## Database Schema (stripe)

### Tables

#### stripe_api_logs
| Column           | Type      | Default           | Constraints         |
|------------------|-----------|-------------------|---------------------|
| id               | uuid      | gen_random_uuid() | PK, NOT NULL        |
| endpoint         | varchar(500)|                  | NOT NULL            |
| method           | varchar(10)|                   | NOT NULL            |
| request_body     | text      |                   |                     |
| response_body    | text      |                   |                     |
| status_code      | integer   |                   |                     |
| error_message    | text      |                   |                     |
| stripe_request_id| varchar(255)|                  |                     |
| created_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### stripe_payment_intents
| Column                 | Type      | Default           | Constraints         |
|------------------------|-----------|-------------------|---------------------|
| id                     | uuid      | gen_random_uuid() | PK, NOT NULL        |
| payment_id             | uuid      |                   | NOT NULL            |
| stripe_payment_intent_id| varchar(255)|                 | NOT NULL            |
| amount                 | bigint    |                   | NOT NULL            |
| currency               | varchar(3)|                   | NOT NULL            |
| status                 | varchar   |                   | NOT NULL            |
| client_secret          | varchar(255)|                  |                     |
| metadata               | json      |                   |                     |
| created_at             | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at             | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### stripe_refunds
| Column                 | Type      | Default           | Constraints         |
|------------------------|-----------|-------------------|---------------------|
| id                     | uuid      | gen_random_uuid() | PK, NOT NULL        |
| payment_id             | uuid      |                   | NOT NULL            |
| stripe_refund_id       | varchar(255)|                  | UNIQUE, NOT NULL    |
| stripe_payment_intent_id| varchar(255)|                 | NOT NULL            |
| amount                 | bigint    |                   | NOT NULL            |
| currency               | varchar(3)|                   | NOT NULL            |
| status                 | varchar   |                   | NOT NULL            |
| reason                 | varchar(50)|                   |                     |
| metadata               | json      |                   |                     |
| created_at             | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at             | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### stripe_webhook_events
| Column           | Type      | Default           | Constraints         |
|------------------|-----------|-------------------|---------------------|
| id               | uuid      | gen_random_uuid() | PK, NOT NULL        |
| stripe_event_id  | varchar(255)|                  | UNIQUE, NOT NULL    |
| event_type       | varchar(100)|                  | NOT NULL            |
| event_data       | json      |                   | NOT NULL            |
| processed        | boolean   | false             | NOT NULL            |
| processing_error | text      |                   |                     |
| created_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

