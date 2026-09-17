# Booking Service API

Manages multi-type bookings: a single generalized model that supports many
booking domains (events, hotel rooms, car rentals, appointments, workshops,
waitlists, on-demand delivery, permits, and more) instead of being tied to
events.

## Location
- API crate: `apis/booking/`
- Feature crates: `features/booking/{entities,model,repo,service,migrations}`
- Service key: `BOOKING`

## Core Concepts

A booking has two classifiers:

| Field | Meaning |
|-------|---------|
| `booking_type` | Free-form domain label: `EVENT`, `HOTEL_ROOM`, `CAR_RENTAL`, `APPOINTMENT`, `WORKSHOP`, `RESTAURANT_WAITLIST`, `FOOD_DELIVERY`, ... |
| `booking_mode` | Internal behavior/shape that selects the mode-specific child table and reservation strategy |

What is being booked is expressed polymorphically via `resource_type` +
`resource_id` (the target lives in another service). Concrete units within a
booking (seats, tables, rooms, cars, slots) are stored as **booking items**.

When a target has no UUID (e.g. a slug or vendor id from a non-native service),
use `external_ref` (an opaque string) instead of `resource_id`. `resource_id`
stays the native UUID path; `external_ref` is the escape hatch — typically only
one of the two identifies a given target. Note that CAPACITY promotion needs a
UUID container, so CAPACITY bookings must use `resource_id`, not `external_ref`.

### Booking Modes

| Mode | Use cases | Child data | Availability guard |
|------|-----------|-----------|--------------------|
| `WINDOW` | car/room/table/desk rentals, appointments | `starts_at`, `ends_at`, `party_size` | Overlap check on `(resource_type, resource_id)` time range |
| `CAPACITY` | airline seats, workshops, event tickets, transit seats | `container_id`, `quantity` | Counter check against `metadata.capacity_limit` |
| `RECURRENCE` | weekly cleaning, recurring desks, passes, permits | `rrule`, `valid_from`, `valid_to` | — |
| `APPROVAL` | request-to-book, emergency plumbing, gated beta | `approval_status`, `hold_expires_at` | — (host-approval workflow) |
| `QUEUE` | restaurant waitlists, beta waitlists | `queue_key`, `position` (auto-assigned) | — (ordered position) |
| `DISPATCH` | food/courier delivery, on-demand plumbing | `dispatch_state`, `pickup_location`, `dropoff_location` | — (provider assignment) |

A create request must include exactly one mode block matching `booking_mode`.

## Endpoints

### Bookings
- `POST /bookings` — Create a booking (Auth: `CanCreateBooking`)
- `GET /bookings` — List/filter bookings with pagination (Auth: `CanReadBooking`)
- `GET /bookings/{booking_id}` — Get booking by ID (Auth: `CanReadBooking`)
- `PATCH /bookings/{booking_id}` — Update a booking (Auth: `CanUpdateBooking`)
- `DELETE /bookings/{booking_id}` — Cancel a booking, **soft-delete** (Auth: `CanDeleteBooking`)
- `GET /bookings/{booking_id}/history` — Booking lifecycle history (Auth: `CanReadBooking`)

### Booking Items
- `POST /booking-items` — Create a line item (Auth: `CanCreateItem`)
- `GET /booking-items` — List/filter items (Auth: `CanReadItem`)
- `GET /booking-items/{booking_item_id}` — Get item by ID (Auth: `CanReadItem`)
- `PATCH /booking-items/{booking_item_id}` — Update an item (Auth: `CanUpdateItem`)
- `DELETE /booking-items/{booking_item_id}` — Delete an item (Auth: `CanDeleteItem`)

### Guest Bookings

Bookings made by unauthenticated guests, later promoted into real bookings after
token-gated confirmation and OAuth-authenticated payment. Like core bookings,
guest bookings are **not event-specific**: they carry `booking_type`,
`booking_mode`, and a polymorphic `resource_type` + `resource_id`. See
`dev/guest-booking-flow.md` for the full flow, security model, and timed windows.

**Public flow** (no auth; `/public/*` is exempt from the baggage requirement):
- `POST /public/guest-bookings` — Create a guest booking (one item per unit)
- `GET /public/guest-bookings/{id}` — Get a guest booking (`?includes=items`); never returns the token/hash
- `POST /public/guest-bookings/{id}/confirm` — Confirm within the window (requires the one-time `confirm_token`)

**Authenticated promotion:**
- `POST /guest-bookings/{id}/promote` — Promote to a real booking after payment (Auth: `CanCreateBooking`)

**Admin management** (authenticated CRUD + search + history):
- `POST /guest-bookings` — Create a guest booking directly, bypassing the confirm-token/email flow (Auth: `CanCreateGuestBooking`)
- `GET /guest-bookings` — List/filter guest bookings with pagination (Auth: `CanReadGuestBooking`)
- `GET /guest-bookings/{id}` — Get a guest booking by ID (Auth: `CanReadGuestBooking`)
- `PATCH /guest-bookings/{id}` — Update a guest booking (Auth: `CanUpdateGuestBooking`)
- `DELETE /guest-bookings/{id}` — Cancel a guest booking, **soft-delete** → `CANCELLED` (Auth: `CanDeleteGuestBooking`)
- `GET /guest-bookings/{id}/history` — Guest booking lifecycle history (Auth: `CanReadGuestBooking`)

Guest-booking history is an append-only log (table `guest_booking_history`)
recording every lifecycle transition — `CREATED`, `CONFIRMED`, `PROMOTED`,
`EXPIRED`, `PAYMENT_EXPIRED` (from the public/promote flow) and `STATUS_CHANGED`
/ `UPDATED` / `CANCELLED` (from admin actions, with `actor_id` set to the admin
user). Read it the same way as booking history:
```
GET /guest-bookings/{id}/history?page=1&page_size=20&order_name=created_at&order_direction=0
```
Returns `QueryResultResponse<GuestBookingHistoryData>` (same shape as
`BookingHistoryData` but keyed by `guest_booking_id`).

## Create Request

### Common fields
| Field | Type | Notes |
|-------|------|-------|
| `booking_type` | String (1–50) | domain label |
| `booking_mode` | enum | `WINDOW`/`CAPACITY`/`RECURRENCE`/`APPROVAL`/`QUEUE`/`DISPATCH` |
| `resource_type` | String? | e.g. `room`, `flight`, `car` |
| `resource_id` | UUID? | id in the owning service |
| `external_ref` | String? | opaque non-UUID target reference; use instead of `resource_id` for resources that have no UUID |
| `user_id` | UUID | |
| `total_amount` | f32 (>= 0) | |
| `currency` | String (3) | ISO 4217 |
| `status` | String (1–50) | e.g. `PENDING` |
| `booking_reference` | String (1–100) | |
| `metadata` | JSON? | free-form extras; `capacity_limit` here enables the CAPACITY guard |
| `window`/`capacity`/`recurrence`/`approval`/`queue`/`dispatch` | object? | supply the one matching `booking_mode` |

### Mode block fields
- `window`: `starts_at` (datetime), `ends_at` (datetime), `party_size` (int?)
- `capacity`: `container_id` (UUID), `quantity` (int, default 1)
- `recurrence`: `rrule` (String?), `valid_from` (datetime), `valid_to` (datetime?)
- `approval`: `hold_expires_at` (datetime?) — status starts `PENDING`
- `queue`: `queue_key` (String) — `position` auto-assigned per queue
- `dispatch`: `pickup_location` (JSON?), `dropoff_location` (JSON?) — state starts `REQUESTED`

### Example — WINDOW (hotel room)
```json
POST /bookings
{
  "booking_type": "HOTEL_ROOM",
  "booking_mode": "WINDOW",
  "resource_type": "room",
  "resource_id": "12345678-1234-1234-1234-123456789022",
  "user_id": "87654321-4321-4321-4321-210987654221",
  "total_amount": 150.50,
  "currency": "USD",
  "status": "PENDING",
  "booking_reference": "BOOKING-WINDOW-001",
  "window": {
    "starts_at": "2026-10-01T14:00:00",
    "ends_at": "2026-10-03T11:00:00",
    "party_size": 2
  }
}
```

### Example — CAPACITY (airline seat, with limit)
```json
POST /bookings
{
  "booking_type": "AIRLINE_SEAT",
  "booking_mode": "CAPACITY",
  "user_id": "87654321-4321-4321-4321-210987654221",
  "total_amount": 320.00,
  "currency": "USD",
  "status": "PENDING",
  "booking_reference": "BOOKING-CAPACITY-001",
  "metadata": { "capacity_limit": 180 },
  "capacity": {
    "container_id": "22222222-2222-2222-2222-222222222222",
    "quantity": 2
  }
}
```

Response (all create endpoints): `OkUuidResponse` → `{ "ok": true, "id": "<uuid>" }`.

## Transaction & Availability Semantics

`BookingService::create_booking` runs everything in one SeaORM transaction:
1. Validate the mode block.
2. Insert the core `bookings` row.
3. Run the mode's availability guard (WINDOW overlap / CAPACITY counter), then
   insert the mode child row (QUEUE also assigns the next position).
4. Commit.

Guards are transaction-scoped. Under high concurrency they are not strictly
race-proof; a PostgreSQL `EXCLUDE USING gist` constraint (WINDOW) or serializable
isolation / `SELECT ... FOR UPDATE` (CAPACITY) is the recommended hardening.

## Lifecycle History

Every meaningful change to a booking is recorded in an append-only
`booking_history` event log, written **in the same transaction** as the change
so the timeline never drifts from booking state.

### Recorded events
| `event_type` | When | Fields captured |
|--------------|------|-----------------|
| `CREATED` | `POST /bookings` | `to_status` = initial status, `actor_id` |
| `STATUS_CHANGED` | `PATCH` when `status` changes | `from_status`, `to_status`, `actor_id` |
| `PAYMENT_UPDATED` | `PATCH` when `payment_status` changes | `note` (old → new), `actor_id` |
| `CANCELLED` | `DELETE` (soft-delete) | `from_status`, `to_status` = `CANCELLED`, `actor_id` |

`actor_id` is the authenticated user (from the baggage `user_id`).

### Soft-delete
`DELETE /bookings/{id}` does **not** physically remove the booking. It sets
`status = CANCELLED` and appends a `CANCELLED` history event, preserving the
booking row and its full history for audit. The response is unchanged
(`{ "ok": true, "id": "<uuid>" }`).

### Reading history
```
GET /bookings/{id}/history?page=1&page_size=20&order_name=created_at&order_direction=0
```
Returns `QueryResultResponse<BookingHistoryData>`:

| Field | Type |
|-------|------|
| `id` | UUID |
| `booking_id` | UUID |
| `event_type` | String |
| `from_status` | String? |
| `to_status` | String? |
| `actor_id` | UUID? |
| `note` | String? |
| `metadata` | JSON? |
| `created_at` | DateTime |

Standard filtering applies, e.g. `?event_type=eq|STATUS_CHANGED`.

## Permissions

| Resource | Permissions |
|----------|-------------|
| `BOOKING:BOOKING` | CREATE, READ, UPDATE, DELETE |
| `BOOKING:ITEM` | CREATE, READ, UPDATE, DELETE |
| `BOOKING:GUEST_BOOKING` | CREATE, READ, UPDATE, DELETE (admin management of guest bookings) |

## Query Parameters

Standard pagination, ordering, field selection, and column filtering apply:
- `?booking_type=eq|HOTEL_ROOM`
- `?status=eq|PENDING`
- `?resource_type=eq|room&resource_id=eq|<uuid>`
- `?booking_id=eq|<uuid>` (on `/booking-items`)
- `?price=lte|300` (on `/booking-items`)
- `?guest_email=eq|guest@example.com` (on `/guest-bookings`)
- `?event_type=eq|CONFIRMED` (on `/guest-bookings/{id}/history`)
- `?page=1&page_size=20&order_name=created_at&order_direction=1`

### Eager-loading related data (`?includes=`)

`GET /bookings` and `GET /bookings/{id}` support eager-loading of the line items
and the mode-specific child row via `includes`:

| Include | Nested field added to `BookingData` |
|---------|--------------------------------------|
| `items` | `items[]` — booking line items |
| `window` | `window` — WINDOW mode data |
| `capacity` | `capacity` — CAPACITY mode data |
| `recurrence` | `recurrence` — RECURRENCE mode data |
| `approval` | `approval` — APPROVAL mode data |
| `queue` | `queue` — QUEUE mode data |
| `dispatch` | `dispatch` — DISPATCH mode data |

Examples:
```
GET /bookings/{id}?includes=window
GET /bookings/{id}?includes=items,window
GET /bookings?includes=capacity
```

Nested fields are omitted from the response unless requested. Typically only the
mode matching the booking's `booking_mode` is worth including; each include adds
one relation-load query.


## Integrations
- Payment / Wallet — via `payment_id` / `payment_status` on the booking
- Resource-owning services (event, inventory, etc.) referenced by `resource_type` + `resource_id`
