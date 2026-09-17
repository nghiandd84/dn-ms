# Guest Booking Flow

Unauthenticated booking with token-gated confirmation, timed windows, and
promotion into a real booking after OAuth-authenticated payment. Part of the
`booking` feature; reached through the gateway at `/api/booking`.

Guest bookings are **not tied to events**: like core bookings they carry a
`booking_type` (domain label), a `booking_mode` (promotion strategy, default
CAPACITY), and a polymorphic target `resource_type` + `resource_id`. So a guest
can book an event, a hotel room, a car, an appointment, etc. For targets without
a UUID, an opaque `external_ref` string can be supplied instead of `resource_id`.

## Overview

A guest (no account) books one or more units, confirms via an emailed one-time
token, authenticates with OAuth to pay, and the guest booking is then
**promoted** into a real `bookings` + `booking_items` record. One booking item
is created per selected unit.

```
Guest (site-a.com)
  │  POST /api/booking/public/guest-bookings         (public, no auth)
  ▼
guest_bookings (PENDING)  ── one guest_booking_item per unit
  │  Kafka: booking topic  event_type=guest_booking_confirm_token
  ▼  (email consumer — separate feature — emails the confirm link)
Guest clicks emailed link → site confirm page
  │  POST /api/booking/public/guest-bookings/{id}/confirm  { confirm_token }
  ▼
guest_bookings (CONFIRMED, confirmed_at set, payment window starts)
  │  OAuth authorization-code flow via /api/auth  → access token
  │  payment (authenticated)
  ▼
  POST /api/booking/guest-bookings/{id}/promote     (AUTH: Auth<CanCreateBooking>)
  ▼
bookings (booking_type / booking_mode / resource, CONFIRMED) + one booking_item per unit + history
guest_bookings (PROMOTED, promoted_booking_id set)
```

Administrators can also manage guest bookings directly (create/search/update/
cancel) and read their history — see **Admin management** below.

## Endpoints

### Public + promotion

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| POST | `/public/guest-bookings` | Public | Create guest booking (one item per unit) |
| GET | `/public/guest-bookings/{id}` | Public | Get guest booking (`?includes=items`); never returns the token/hash |
| POST | `/public/guest-bookings/{id}/confirm` | Public + confirm token | Confirm within the 20-min window |
| POST | `/guest-bookings/{id}/promote` | `Auth<CanCreateBooking>` | Promote to a real booking after payment |

`/public/*` routes are exempt from the `require_baggage_header` middleware, so
they need no `baggage`/JWT. The promote route is NOT under `/public` and
requires the gateway-injected baggage (real `user_id`).

### Admin management

Authenticated CRUD + search + history over guest bookings, guarded by the
`BOOKING:GUEST_BOOKING` permission resource.

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| POST | `/guest-bookings` | `Auth<CanCreateGuestBooking>` | Create directly, bypassing the confirm-token/email flow |
| GET | `/guest-bookings` | `Auth<CanReadGuestBooking>` | List/filter with pagination |
| GET | `/guest-bookings/{id}` | `Auth<CanReadGuestBooking>` | Get by ID |
| PATCH | `/guest-bookings/{id}` | `Auth<CanUpdateGuestBooking>` | Update |
| DELETE | `/guest-bookings/{id}` | `Auth<CanDeleteGuestBooking>` | Cancel (soft-delete → `CANCELLED`) |
| GET | `/guest-bookings/{id}/history` | `Auth<CanReadGuestBooking>` | Lifecycle history (paged) |

Admin create is transactional and takes no confirmation token (the record is
created with an explicit `status`, default `PENDING`); it generates a
`booking_reference` and `expires_at` if not supplied. Admin delete is a
**soft-delete**: it sets `status = CANCELLED` and records a `CANCELLED` history
event rather than removing the row.

## Data model

Tables in the `booking` feature (`features/booking/entities`), created by
migration `m20260216_000001_create_guest_booking_tables` (guest booking + items)
and `m20260218_000001_create_guest_booking_history` (history).

**`guest_bookings`**

| Column | Notes |
|--------|-------|
| `id` (uuid, pk) | |
| `booking_type` (string) | domain label, e.g. EVENT, HOTEL_ROOM, CAR_RENTAL; default EVENT |
| `booking_mode` (string) | promotion strategy: WINDOW/CAPACITY/RECURRENCE/APPROVAL/QUEUE/DISPATCH; default CAPACITY |
| `resource_type` (string, null) | kind of target: event, room, car, ... |
| `resource_id` (uuid, null) | concrete target id in the owning service |
| `external_ref` (string, null) | opaque non-UUID target reference; used instead of `resource_id` for non-native resources |
| `site_origin` (string) | booking site origin, allowlist-validated |
| `confirm_path` (string) | path used to build the confirm link |
| `guest_email` (string) | |
| `guest_name` (string, null) | |
| `total_amount` (float) | computed from unit prices (public flow) |
| `currency` (string) | ISO 4217 |
| `status` (string) | PENDING → CONFIRMED → PROMOTED; or EXPIRED / PAYMENT_EXPIRED / CANCELLED |
| `booking_reference` (string) | generated, e.g. `GBK-1A2B3C4D` |
| `confirm_token_hash` (string(64), null) | SHA-256 hex of the one-time token; plaintext never stored |
| `expires_at` (datetime) | confirm deadline = `created_at + 20min` |
| `payment_expires_at` (datetime, null) | payment deadline = `confirmed_at + 60min` (set at confirm) |
| `metadata` (jsonb, null) | |
| `promoted_booking_id` (uuid, null) | set on promotion |
| `created_at` / `updated_at` / `confirmed_at` | |

Indexes: `(resource_type, resource_id)`, `status`, `booking_reference`,
`(status, expires_at)`.

**`guest_booking_items`** — one row per unit: `id, guest_booking_id (fk cascade),
item_type ("seat" by default), item_id, price, metadata, created_at, updated_at`.

**`guest_booking_history`** — append-only lifecycle log: `id,
guest_booking_id (fk cascade), event_type, from_status, to_status,
actor_id (admin user for admin changes), note, metadata, created_at`. Indexed on
`(guest_booking_id, created_at)`. Written in the same transaction as the change
it records. Event types: `CREATED`, `CONFIRMED`, `PROMOTED`, `EXPIRED`,
`PAYMENT_EXPIRED`, `STATUS_CHANGED`, `UPDATED`, `CANCELLED`.

## Security

### Confirmation token (token-gated confirm)
The confirm endpoint is public (no OAuth — guests have no account) but is
authorized by a **one-time capability token**, because knowing the booking UUID
alone must not be enough to confirm.

- Generated at create: 256 bits from a CSPRNG (`rand::thread_rng().fill_bytes`).
- Only the **SHA-256 hash** is stored (`confirm_token_hash`); plaintext is never
  persisted and never returned over HTTP.
- The plaintext token is published to Kafka (see below) for out-of-band email
  delivery.
- At confirm, the presented token is hashed and compared to the stored hash in
  **constant time**; mismatch → `AuthError::InsufficientPermission`.

### Site-origin allowlist (open-redirect protection)
The confirm link points back to the site the guest booked from. To prevent
host-injection / open-redirect abuse:

- The client sends `site_origin` (+ optional `confirm_path`, default
  `/path/confirm_booking`).
- The service normalizes it (trim, lowercase, strip trailing slash) and checks
  it against `GUEST_BOOKING_ALLOWED_ORIGINS` (comma-separated env allowlist).
  **Fails closed**: unset/empty ⇒ nothing allowed.
- The normalized origin is persisted; the confirm URL is built server-side:
  `{site_origin}{confirm_path}?guest_booking_id=<id>&token=<confirm_token>`.

### Create-endpoint abuse protection
Public write endpoint, so layered throttling:

- **Gateway rate limiter** on `booking_router_filter` (token bucket per client
  IP). See `docker/gateway/dn-config.yaml` (`booking_rate_limiter`).
- **Service caps** (IP-independent, in `GuestBookingService`):
  - `MAX_SEATS_PER_BOOKING = 10` — reject oversized requests.
  - `MAX_ACTIVE_PENDING_PER_EMAIL = 5` within
    `PENDING_THROTTLE_WINDOW_MINUTES = 60` — limits spam / email bombing, via
    `GuestBookingQuery::count_active_pending_by_email_since`.

CORS (`allowed_domains`) is a browser control, not a security boundary.

## Timed windows

Enforced lazily (checked at the point of action; no background sweeper).

| Window | Constant | Set at | Enforced at | On expiry |
|--------|----------|--------|-------------|-----------|
| Confirm (20 min) | `CONFIRM_WINDOW_MINUTES = 20` | create (`expires_at`) | confirm | status → EXPIRED, reject |
| Payment (60 min) | `PAYMENT_WINDOW_MINUTES = 60` | confirm (`payment_expires_at = confirmed_at + 60min`) | promote | status → PAYMENT_EXPIRED, reject |

Because expiry is lazy, a booking flips to EXPIRED / PAYMENT_EXPIRED only when a
confirm/promote is attempted after the deadline. The `(status, expires_at)`
index supports adding an active sweeper later if desired.

## Kafka event

On create (AFTER the DB commit), the service publishes to the booking topic via
the shared `Producer` (registered under `PRODUCER_KEY = "booking"` in
`api-booking` `app.rs` `custom_handler`, using env
`BOOKING_KAFKA_BOOTSTRAP_SERVERS` / `BOOKING_KAFKA_TOPIC`).

Message (`features/booking/stream`): `BookingMessage`, serde-tagged by
`event_type`:

```json
{
  "event_type": "guest_booking_confirm_token",
  "message": {
    "guest_booking_id": "…",
    "resource_type": "event",
    "resource_id": "…",
    "external_ref": null,
    "guest_email": "guest@example.com",
    "guest_name": "Jane Guest",
    "booking_reference": "GBK-1A2B3C4D",
    "confirm_token": "<plaintext one-time token>",
    "confirm_url": "https://site-a.com/path/confirm_booking?guest_booking_id=…&token=…",
    "expires_at": "2026-09-17T04:20:00+00:00"
  }
}
```

A separate consumer feature subscribes and emails the `confirm_url` to
`guest_email`. Publishing is best-effort-with-error: the booking is already
committed; a publish failure surfaces as an error to the caller.

## Promotion

`GuestBookingService::promote_guest_booking(guest_booking_id, user_id)` runs in
one transaction:

1. Load + validate: status must be `CONFIRMED`, not already promoted, payment
   window not elapsed (else → PAYMENT_EXPIRED).
2. Create core `bookings` row carrying the guest booking's own classification and
   target: `booking_type`, `booking_mode`, `resource_type`, `resource_id`,
   `user_id` (authenticated), `status=CONFIRMED`, `payment_status=SUCCESS`,
   generated reference.
3. Mode child row: for `booking_mode = CAPACITY` (the default) create the
   capacity row (`container_id = resource_id`, `quantity = #units`). CAPACITY
   promotion therefore **requires a `resource_id`** (rejected with an error if
   missing). Other modes are carried on the core row but do not yet create a
   child row (future work).
4. Create one `booking_item` per guest unit item.
5. Append a `booking_history` CREATED entry noting the source guest booking.
6. Mark the guest booking `PROMOTED`, set `promoted_booking_id`, and append a
   `guest_booking_history` `PROMOTED` entry.

## Configuration

| Env | Used by | Purpose |
|-----|---------|---------|
| `GUEST_BOOKING_ALLOWED_ORIGINS` | booking service | Comma-separated allowlist of site origins, e.g. `https://site-a.com,https://site-b.com` |
| `BOOKING_KAFKA_BOOTSTRAP_SERVERS` | api-booking | Kafka brokers for the producer |
| `BOOKING_KAFKA_TOPIC` | api-booking | Booking topic name |

Gateway: `booking_rate_limiter` interceptor on `booking_router_filter` in
`docker/gateway/dn-config.yaml`.

## Code map

| Layer | Path |
|-------|------|
| Entities | `features/booking/entities/src/guest_booking.rs`, `guest_booking_item.rs`, `guest_booking_history.rs` |
| Migration | `features/booking/migrations/src/m20260216_000001_create_guest_booking_tables.rs`, `m20260218_000001_create_guest_booking_history.rs` |
| Model | `features/booking/model/src/guest_booking.rs`, `guest_booking_item.rs`, `guest_booking_history.rs` |
| Repo | `features/booking/repo/src/guest_booking/`, `guest_booking_item/`, `guest_booking_history/` |
| Service | `features/booking/service/src/guest_booking.rs` (`GuestBookingService`) |
| Stream | `features/booking/stream/src/lib.rs` (`BookingMessage`, `PRODUCER_KEY`) |
| API routes | `apis/booking/src/routes/guest_booking.rs` |
| App wiring | `apis/booking/src/app.rs` (producer), `doc.rs` (OpenAPI) |
| REST samples | `apis/booking/test.rest` (Guest Booking section) |

## Notes / follow-ups

- Expiry is lazy; add a periodic sweeper in `custom_handler` if you need seats
  freed proactively.
- The per-email throttle keys on `guest_email` (attacker can vary it); the
  gateway IP limiter is the volume cap. CAPTCHA is the strongest anti-bot option
  if added later.
- The confirm link carries the token as a query param; consider a short-lived
  opaque code or immediate POST-and-strip if query-string token leakage
  (history/referrer/logs) is a concern.
