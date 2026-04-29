# PayPal Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    paypal_api_logs {
        uuid id PK
        varchar endpoint
        varchar method
        text request_body
        text response_body
        integer status_code
        text error_message
        varchar paypal_request_id
        timestamp created_at
        timestamp updated_at
    }
    paypal_orders {
        uuid id PK
        uuid payment_id
        varchar paypal_order_id
        bigint amount
        varchar currency
        varchar status
        varchar approval_url
        varchar capture_id
        json metadata
        timestamp created_at
        timestamp updated_at
    }
    paypal_refunds {
        uuid id PK
        uuid payment_id
        varchar paypal_refund_id UNIQUE
        varchar paypal_capture_id
        bigint amount
        varchar currency
        varchar status
        varchar reason
        json metadata
        timestamp created_at
        timestamp updated_at
    }
    paypal_webhook_events {
        uuid id PK
        varchar paypal_event_id UNIQUE
        varchar event_type
        json event_data
        boolean processed
        text processing_error
        timestamp created_at
        timestamp updated_at
    }
```

## Database Schema (paypal)

### Tables

#### paypal_orders
| Column           | Type         | Default           | Constraints         |
|------------------|--------------|-------------------|---------------------|
| id               | uuid         | gen_random_uuid() | PK, NOT NULL        |
| payment_id       | uuid         |                   | NOT NULL            |
| paypal_order_id  | varchar(255) |                   | NOT NULL            |
| amount           | bigint       |                   | NOT NULL            |
| currency         | varchar(3)   |                   | NOT NULL            |
| status           | varchar      |                   | NOT NULL            |
| approval_url     | varchar(2000)|                   |                     |
| capture_id       | varchar(255) |                   |                     |
| metadata         | json         |                   |                     |
| created_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |

---

#### paypal_refunds
| Column           | Type         | Default           | Constraints         |
|------------------|--------------|-------------------|---------------------|
| id               | uuid         | gen_random_uuid() | PK, NOT NULL        |
| payment_id       | uuid         |                   | NOT NULL            |
| paypal_refund_id | varchar(255) |                   | UNIQUE, NOT NULL    |
| paypal_capture_id| varchar(255) |                   | NOT NULL            |
| amount           | bigint       |                   | NOT NULL            |
| currency         | varchar(3)   |                   | NOT NULL            |
| status           | varchar      |                   | NOT NULL            |
| reason           | varchar(500) |                   |                     |
| metadata         | json         |                   |                     |
| created_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |

---

#### paypal_webhook_events
| Column           | Type         | Default           | Constraints         |
|------------------|--------------|-------------------|---------------------|
| id               | uuid         | gen_random_uuid() | PK, NOT NULL        |
| paypal_event_id  | varchar(255) |                   | UNIQUE, NOT NULL    |
| event_type       | varchar(100) |                   | NOT NULL            |
| event_data       | json         |                   | NOT NULL            |
| processed        | boolean      | false             | NOT NULL            |
| processing_error | text         |                   |                     |
| created_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |

---

#### paypal_api_logs
| Column           | Type         | Default           | Constraints         |
|------------------|--------------|-------------------|---------------------|
| id               | uuid         | gen_random_uuid() | PK, NOT NULL        |
| endpoint         | varchar(500) |                   | NOT NULL            |
| method           | varchar(10)  |                   | NOT NULL            |
| request_body     | text         |                   |                     |
| response_body    | text         |                   |                     |
| status_code      | integer      |                   |                     |
| error_message    | text         |                   |                     |
| paypal_request_id| varchar(255) |                   |                     |
| created_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at       | timestamp    | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

## Indexes

| Index Name | Table | Column(s) | Notes |
|------------|-------|-----------|-------|
| idx_paypal_order_payment_id | paypal_orders | payment_id | |
| idx_paypal_order_paypal_order_id | paypal_orders | paypal_order_id | Partial: WHERE NOT NULL |
| idx_paypal_refund_payment_id | paypal_refunds | payment_id | |
| idx_paypal_refund_capture_id | paypal_refunds | paypal_capture_id | |
| idx_paypal_webhook_event_type | paypal_webhook_events | event_type | |
| idx_paypal_webhook_processed | paypal_webhook_events | processed | |
| idx_paypal_api_log_endpoint | paypal_api_logs | endpoint | |
| idx_paypal_api_log_created_at | paypal_api_logs | created_at | |
