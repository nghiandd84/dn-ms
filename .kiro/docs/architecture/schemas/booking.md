# Booking Microservice Database Schema

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    bookings {
        uuid id PK
        uuid event_id
        uuid user_id
        real total_amount
        varchar status
        uuid payment_id
        varchar payment_status
        varchar booking_reference UNIQUE
        timestamp created_at
        timestamp updated_at
        timestamp confirmed_at
        varchar currency
    }
    booking_seats {
        uuid id PK
        uuid booking_id FK
        uuid seat_id
        real price
        timestamp created_at
        timestamp updated_at
    }
    seaql_migrations {
        varchar version PK
        bigint applied_at
    }
    bookings ||--o{ booking_seats : "has seats"
```

## Database Schema (booking)

### Tables

#### bookings
| Column             | Type      | Default                | Constraints         |
|--------------------|-----------|------------------------|---------------------|
| id                 | uuid      | gen_random_uuid()      | PK, NOT NULL        |
| event_id           | uuid      |                        | NOT NULL            |
| user_id            | uuid      |                        | NOT NULL            |
| total_amount       | real      |                        | NOT NULL            |
| status             | varchar   | 'PENDING'              | NOT NULL            |
| payment_id         | uuid      |                        |                     |
| payment_status     | varchar   |                        |                     |
| booking_reference  | varchar(50)|                       | UNIQUE              |
| created_at         | timestamp | CURRENT_TIMESTAMP      | NOT NULL            |
| updated_at         | timestamp | CURRENT_TIMESTAMP      | NOT NULL            |
| confirmed_at       | timestamp |                        |                     |
| currency           | varchar   | 'USD'                  | NOT NULL            |

---

#### booking_seats
| Column      | Type      | Default           | Constraints         |
|-------------|-----------|-------------------|---------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL        |
| booking_id  | uuid      |                   | NOT NULL, FK        |
| seat_id     | uuid      |                   | NOT NULL            |
| price       | real      |                   | NOT NULL            |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL            |

---

#### seaql_migrations
| Column      | Type      | Default | Constraints         |
|-------------|-----------|---------|---------------------|
| version     | varchar   |         | PK                  |
| applied_at  | bigint    |         | NOT NULL            |

