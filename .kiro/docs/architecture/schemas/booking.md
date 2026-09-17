# Booking Microservice Database Schema

Generalized multi-type booking schema: a core `bookings` table, a `booking_items`
line-item table, and one child table per booking mode. `booking_mode` selects
which child table applies to a given booking.

## Entity Relationship Diagram (Mermaid)

```mermaid
erDiagram
    bookings {
        uuid id PK
        varchar booking_type
        varchar booking_mode
        varchar resource_type
        uuid resource_id
        uuid user_id
        real total_amount
        varchar currency
        varchar status
        uuid payment_id
        varchar payment_status
        varchar booking_reference
        jsonb metadata
        int version
        timestamp created_at
        timestamp updated_at
        timestamp confirmed_at
    }
    booking_items {
        uuid id PK
        uuid booking_id FK
        varchar item_type
        uuid item_id
        real price
        jsonb metadata
        timestamp created_at
        timestamp updated_at
    }
    booking_windows {
        uuid booking_id PK "FK"
        timestamp starts_at
        timestamp ends_at
        int party_size
    }
    booking_capacity {
        uuid booking_id PK "FK"
        uuid container_id
        int quantity
    }
    booking_recurrence {
        uuid booking_id PK "FK"
        varchar rrule
        timestamp valid_from
        timestamp valid_to
    }
    booking_approval {
        uuid booking_id PK "FK"
        varchar approval_status
        uuid approver_id
        timestamp requested_at
        timestamp decided_at
        timestamp hold_expires_at
        varchar reason
    }
    booking_queue {
        uuid booking_id PK "FK"
        varchar queue_key
        int position
        timestamp estimated_ready
        timestamp offered_at
        timestamp hold_expires_at
    }
    booking_dispatch {
        uuid booking_id PK "FK"
        varchar dispatch_state
        uuid provider_id
        timestamp assigned_at
        jsonb pickup_location
        jsonb dropoff_location
        timestamp eta
    }
    booking_history {
        uuid id PK
        uuid booking_id FK
        varchar event_type
        varchar from_status
        varchar to_status
        uuid actor_id
        varchar note
        jsonb metadata
        timestamp created_at
    }
    guest_bookings {
        uuid id PK
        uuid event_id
        varchar site_origin
        varchar confirm_path
        varchar guest_email
        varchar guest_name
        real total_amount
        varchar currency
        varchar status
        varchar booking_reference
        varchar confirm_token_hash
        timestamp expires_at
        timestamp payment_expires_at
        jsonb metadata
        uuid promoted_booking_id
        timestamp created_at
        timestamp updated_at
        timestamp confirmed_at
    }
    guest_booking_items {
        uuid id PK
        uuid guest_booking_id FK
        varchar item_type
        uuid item_id
        real price
        jsonb metadata
        timestamp created_at
        timestamp updated_at
    }
    bookings ||--o{ booking_items : "has items"
    bookings ||--o| booking_windows : "WINDOW"
    bookings ||--o| booking_capacity : "CAPACITY"
    bookings ||--o| booking_recurrence : "RECURRENCE"
    bookings ||--o| booking_approval : "APPROVAL"
    bookings ||--o| booking_queue : "QUEUE"
    bookings ||--o| booking_dispatch : "DISPATCH"
    bookings ||--o{ booking_history : "history"
    guest_bookings ||--o{ guest_booking_items : "has items"
    guest_bookings ||..o| bookings : "promoted_booking_id"
```

## Database Schema (booking)

### Core

#### bookings
| Column             | Type        | Default             | Constraints  |
|--------------------|-------------|---------------------|--------------|
| id                 | uuid        | gen_random_uuid()   | PK, NOT NULL |
| booking_type       | varchar     |                     | NOT NULL     |
| booking_mode       | varchar     |                     | NOT NULL     |
| resource_type      | varchar     |                     | NULL         |
| resource_id        | uuid        |                     | NULL         |
| user_id            | uuid        |                     | NOT NULL     |
| total_amount       | real        | 0                   | NOT NULL     |
| currency           | varchar     | 'USD'               | NOT NULL     |
| status             | varchar     | 'PENDING'           | NOT NULL     |
| payment_id         | uuid        |                     | NULL         |
| payment_status     | varchar     | 'PENDING'           | NOT NULL     |
| booking_reference  | varchar(100)|                     | NOT NULL     |
| metadata           | jsonb       |                     | NULL         |
| version            | int         | 0                   | NOT NULL     |
| created_at         | timestamp   | CURRENT_TIMESTAMP   | NOT NULL     |
| updated_at         | timestamp   | CURRENT_TIMESTAMP   | NOT NULL     |
| confirmed_at       | timestamp   |                     | NULL         |

Indexes: `idx_bookings_user (user_id)`, `idx_bookings_type_status (booking_type, status)`,
`idx_bookings_resource (resource_type, resource_id)`, `idx_bookings_reference (booking_reference)`.

#### booking_items
| Column      | Type      | Default           | Constraints                  |
|-------------|-----------|-------------------|------------------------------|
| id          | uuid      | gen_random_uuid() | PK, NOT NULL                 |
| booking_id  | uuid      |                   | NOT NULL, FK → bookings(id)  |
| item_type   | varchar   |                   | NOT NULL                     |
| item_id     | uuid      |                   | NOT NULL                     |
| price       | real      |                   | NOT NULL                     |
| metadata    | jsonb     |                   | NULL                         |
| created_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL                     |
| updated_at  | timestamp | CURRENT_TIMESTAMP | NOT NULL                     |

Indexes: `idx_booking_items_booking (booking_id)`, `idx_booking_items_unit (item_type, item_id)`.
FK cascade: `ON DELETE CASCADE`.

#### booking_history (append-only lifecycle event log)
| Column      | Type        | Default           | Constraints                 |
|-------------|-------------|-------------------|-----------------------------|
| id          | uuid        | gen_random_uuid() | PK, NOT NULL                |
| booking_id  | uuid        |                   | NOT NULL, FK → bookings(id) |
| event_type  | varchar     |                   | NOT NULL (CREATED, STATUS_CHANGED, PAYMENT_UPDATED, CANCELLED) |
| from_status | varchar     |                   | NULL                        |
| to_status   | varchar     |                   | NULL                        |
| actor_id    | uuid        |                   | NULL                        |
| note        | varchar(500)|                   | NULL                        |
| metadata    | jsonb       |                   | NULL                        |
| created_at  | timestamp   | CURRENT_TIMESTAMP | NOT NULL                    |

Index: `idx_booking_history_booking (booking_id, created_at)`. FK cascade: `ON DELETE CASCADE`.
Append-only: no update/delete operations. Rows are written in the same transaction
as the booking change they record.

### Mode child tables

Each has `booking_id` as PK and FK → `bookings(id)` `ON DELETE CASCADE` (1:1).

#### booking_windows (mode = WINDOW)
| Column     | Type      | Constraints  |
|------------|-----------|--------------|
| booking_id | uuid      | PK, FK       |
| starts_at  | timestamp | NOT NULL     |
| ends_at    | timestamp | NOT NULL     |
| party_size | int       | NULL         |

Index: `idx_booking_windows_range (starts_at, ends_at)`.

#### booking_capacity (mode = CAPACITY)
| Column       | Type | Default | Constraints |
|--------------|------|---------|-------------|
| booking_id   | uuid |         | PK, FK      |
| container_id | uuid |         | NOT NULL    |
| quantity     | int  | 1       | NOT NULL    |

Index: `idx_booking_capacity_container (container_id)`.

#### booking_recurrence (mode = RECURRENCE)
| Column     | Type        | Constraints |
|------------|-------------|-------------|
| booking_id | uuid        | PK, FK      |
| rrule      | varchar(500)| NULL        |
| valid_from | timestamp   | NOT NULL    |
| valid_to   | timestamp   | NULL        |

#### booking_approval (mode = APPROVAL)
| Column          | Type        | Default           | Constraints |
|-----------------|-------------|-------------------|-------------|
| booking_id      | uuid        |                   | PK, FK      |
| approval_status | varchar     | 'PENDING'         | NOT NULL    |
| approver_id     | uuid        |                   | NULL        |
| requested_at    | timestamp   | CURRENT_TIMESTAMP | NOT NULL    |
| decided_at      | timestamp   |                   | NULL        |
| hold_expires_at | timestamp   |                   | NULL        |
| reason          | varchar(500)|                   | NULL        |

#### booking_queue (mode = QUEUE)
| Column          | Type        | Constraints |
|-----------------|-------------|-------------|
| booking_id      | uuid        | PK, FK      |
| queue_key       | varchar(100)| NOT NULL    |
| position        | int         | NULL        |
| estimated_ready | timestamp   | NULL        |
| offered_at      | timestamp   | NULL        |
| hold_expires_at | timestamp   | NULL        |

Index: `idx_booking_queue_key_pos (queue_key, position)`.

#### booking_dispatch (mode = DISPATCH)
| Column           | Type      | Default     | Constraints |
|------------------|-----------|-------------|-------------|
| booking_id       | uuid      |             | PK, FK      |
| dispatch_state   | varchar   | 'REQUESTED' | NOT NULL    |
| provider_id      | uuid      |             | NULL        |
| assigned_at      | timestamp |             | NULL        |
| pickup_location  | jsonb     |             | NULL        |
| dropoff_location | jsonb     |             | NULL        |
| eta              | timestamp |             | NULL        |

Index: `idx_booking_dispatch_provider (provider_id, dispatch_state)`.

### Guest booking

Unauthenticated event booking that is later promoted into a real `bookings` row.
See `dev/guest-booking-flow.md` for the full flow, security model, and windows.

#### guest_bookings
| Column              | Type         | Default           | Constraints  |
|---------------------|--------------|-------------------|--------------|
| id                  | uuid         | gen_random_uuid() | PK, NOT NULL |
| event_id            | uuid         |                   | NOT NULL     |
| site_origin         | varchar      |                   | NOT NULL (allowlist-validated) |
| confirm_path        | varchar(255) |                   | NOT NULL     |
| guest_email         | varchar      |                   | NOT NULL     |
| guest_name          | varchar      |                   | NULL         |
| total_amount        | real         | 0                 | NOT NULL     |
| currency            | varchar      | 'USD'             | NOT NULL     |
| status              | varchar      | 'PENDING'         | NOT NULL (PENDING, CONFIRMED, PROMOTED, CANCELLED, EXPIRED, PAYMENT_EXPIRED) |
| booking_reference   | varchar(100) |                   | NOT NULL     |
| confirm_token_hash  | varchar(64)  |                   | NULL (SHA-256 hex of one-time token; plaintext never stored) |
| expires_at          | timestamp    |                   | NOT NULL (confirm deadline = created_at + 20min) |
| payment_expires_at  | timestamp    |                   | NULL (payment deadline = confirmed_at + 60min) |
| metadata            | jsonb        |                   | NULL         |
| promoted_booking_id | uuid         |                   | NULL (set on promotion → bookings(id)) |
| created_at          | timestamp    | CURRENT_TIMESTAMP | NOT NULL     |
| updated_at          | timestamp    | CURRENT_TIMESTAMP | NOT NULL     |
| confirmed_at        | timestamp    |                   | NULL         |

Indexes: `idx_guest_bookings_event (event_id)`, `idx_guest_bookings_status (status)`,
`idx_guest_bookings_reference (booking_reference)`,
`idx_guest_bookings_status_expires (status, expires_at)`.

#### guest_booking_items (one row per selected seat)
| Column           | Type      | Default           | Constraints                        |
|------------------|-----------|-------------------|------------------------------------|
| id               | uuid      | gen_random_uuid() | PK, NOT NULL                       |
| guest_booking_id | uuid      |                   | NOT NULL, FK → guest_bookings(id)  |
| item_type        | varchar   |                   | NOT NULL (e.g. "seat")             |
| item_id          | uuid      |                   | NOT NULL                           |
| price            | real      |                   | NOT NULL                           |
| metadata         | jsonb     |                   | NULL                               |
| created_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL                           |
| updated_at       | timestamp | CURRENT_TIMESTAMP | NOT NULL                           |

Indexes: `idx_guest_booking_items_guest_booking (guest_booking_id)`,
`idx_guest_booking_items_unit (item_type, item_id)`. FK cascade: `ON DELETE CASCADE`.

Migration: `m20260216_000001_create_guest_booking_tables`.

#### seaql_migrations
| Column     | Type    | Constraints |
|------------|---------|-------------|
| version    | varchar | PK          |
| applied_at | bigint  | NOT NULL    |

## Notes
- `money` fields use `real` (f32) to match legacy columns. Consider migrating to
  integer minor units (`i64`) to align with the payment/wallet services and avoid
  float rounding.
- `version` on `bookings` is reserved for optimistic locking.
- Availability is enforced in the service transaction, not (yet) by DB constraints.
  A `EXCLUDE USING gist` overlap constraint on WINDOW is the recommended hardening.
