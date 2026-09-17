# Implementation Plan — Guest Booking API Changes (dn-ms backend)

## Problem Statement
Add a guest-booking capability to the `booking` feature so unauthenticated guests can create and confirm bookings for events (one booking item per seat). After the guest authenticates (OAuth) and pays, the guest booking is promoted into the real `booking` + `booking_items` records. All endpoints are reached through the API Gateway (`http://localhost:6001/api/booking`).

## Requirements
- Guests create/confirm bookings WITHOUT a token → public routes using the `PublicAccess` extractor, exposed under a `/public/...` path prefix so the `require_baggage_header` middleware exempts them.
- Booking granularity: one booking item per seat.
- Promotion (guest booking → real booking) is an AUTHENTICATED endpoint using `Auth<CanCreateBooking>`, binding the real booking to the authenticated user, run after OAuth + payment.
- Follow existing `booking` feature conventions across the 5 crates: entities, model, repo, service, migrations, plus the `apis/booking` API layer.

## Background (verified in code)
- Gateway `docker/gateway/dn-config.yaml`: `booking` router has CORS only, NO `token_auth`; `auth` router enforces `token_auth`.
- `libs/shared/shared/auth/src/permission.rs`: every non-public route uses `Auth<R>` requiring a `baggage` header (`accesses`, `user_id`, `client_id`). `PublicAccess` marks no-auth routes. `require_baggage_header` treats paths starting with `/public`, `/swagger-ui`, `/api-docs` as infra-exempt.
- Booking feature crates: `features/booking/{entities,model,repo,service,migrations}` and API in `apis/booking`. Entities use the `Dto` macro + `ActiveModelBehavior` timestamps. Repo uses `Mutation`/`Query` macros with `_with_txn` variants. Service uses `DB_WRITE` + transactions. Migrations registered in `features/booking/migrations/src/lib.rs`.
- `booking_items` fields: `id, booking_id, item_type ("seat"), item_id, price, metadata, created_at, updated_at`.
- Existing `BookingMutation::create_booking_with_txn` and CAPACITY child-row creation can be reused during promotion.

## Task Breakdown

### Task 1: Guest-booking entities + migration
- Objective: Add `guest_bookings` and `guest_booking_items` tables and SeaORM entity models to the `booking` feature.
- Guidance: Create `features/booking/entities/src/guest_booking.rs` and `guest_booking_item.rs` mirroring `booking.rs`/`booking_item.rs` (use the `Dto` macro, `ActiveModelBehavior` timestamps). `guest_bookings` fields: `id, event_id, guest_email, guest_name, total_amount, currency, status (PENDING/CONFIRMED/PROMOTED/CANCELLED), booking_reference, metadata, promoted_booking_id (nullable), created_at, updated_at, confirmed_at (nullable)`. `guest_booking_items` fields: `id, guest_booking_id, item_type, item_id (seat), price, metadata, created_at, updated_at`. Add migration `m2026..._create_guest_booking_tables.rs`, register in `migrations/src/lib.rs`, export modules in `entities/src/lib.rs`.
- Tests: Migration up/down runs against a test DB; entity round-trip insert/select in a repo-level test.
- Demo: Run migrations; show the two new tables exist and accept an insert via a SeaORM test.

### Task 2: Guest-booking repo layer (Mutation/Query)
- Objective: Add repo modules for guest bookings and items, including transactional variants.
- Guidance: Create `features/booking/repo/src/guest_booking/{mod,mutation,query,util}.rs` and `guest_booking_item/{mod,mutation,query,util}.rs`, mirroring `booking`/`booking_item` repos (`Mutation`/`Query` macros; add `create_*_with_txn`, `update_*_with_txn`, `list_items_by_guest_booking`). Export in `repo/src/lib.rs`.
- Tests: Repo unit tests for create, get-by-id, list-items-by-guest-booking, update-status against a test DB.
- Demo: Create a guest booking with two seat items and read them back via repo tests.

### Task 3: Guest-booking model layer (requests/responses)
- Objective: Add request/response DTOs with validation and filter params.
- Guidance: Create `features/booking/model/src/guest_booking.rs` and `guest_booking_item.rs` mirroring existing models (`Response`, `ParamFilter` macros; `Validate`). Define `GuestBookingForCreateRequest` (nested `Vec<GuestSeatInput>` for one-item-per-seat), `GuestBookingData`, `GuestBookingConfirmRequest`, `GuestBookingForUpdateRequest`. Export in `model/src/lib.rs`.
- Tests: Serde (de)serialization tests and validator tests (empty seats rejected, currency length = 3).
- Demo: Deserialize a sample create payload with multiple seats and validate it in a unit test.

### Task 4: Guest-booking service (create + confirm)
- Objective: Implement business logic to create a guest booking with one item per seat and to confirm it, transactionally.
- Guidance: Add `features/booking/service/src/guest_booking.rs`. `create_guest_booking` inserts the guest booking + one `guest_booking_item` per seat in a transaction (compute `total_amount` from seat prices; default status PENDING). `confirm_guest_booking` sets status CONFIRMED + `confirmed_at`. Reuse the `DB_WRITE`/`txn` pattern from `booking.rs`. Export in `service/src/lib.rs`.
- Tests: Service tests: create builds N items and correct total; confirm transitions status; confirming a non-existent/promoted booking errors.
- Demo: Create a 3-seat guest booking (PENDING) and confirm it (CONFIRMED) via service test.

### Task 5: Public guest-booking API routes (no auth)
- Objective: Expose guest create/get/confirm as public endpoints through the booking API.
- Guidance: Add `apis/booking/src/routes/guest_booking.rs` using the `PublicAccess` extractor. Routes under `/public/guest-bookings` so `require_baggage_header` exempts them: `POST /public/guest-bookings`, `GET /public/guest-bookings/{id}`, `POST /public/guest-bookings/{id}/confirm`. Merge in `apis/booking/src/app.rs`, add to `doc.rs` OpenAPI, register module in `routes/mod.rs`.
- Tests: Integration tests (per `.kiro/skills/rust/test-integration-dir.md`) hitting routes without a baggage header: create → 201, get → 200, confirm → 200.
- Demo: Via `test.rest` through the gateway (`POST http://localhost:6001/api/booking/public/guest-bookings` with only an `Origin` header) create and confirm a guest booking.

### Task 6: Promotion service (guest booking → real booking)
- Objective: Convert a CONFIRMED guest booking into a real `booking` (CAPACITY mode) with one `booking_item` per seat, atomically, and mark the guest booking PROMOTED.
- Guidance: Add `promote_guest_booking(guest_booking_id, user_id)` in the guest-booking service. In one transaction: load guest booking + items (must be CONFIRMED and not already promoted); create core booking via `BookingMutation::create_booking_with_txn` (booking_type `EVENT`, mode `CAPACITY`, resource_type `event`, resource_id = event_id, user_id, total_amount, currency, status `CONFIRMED`, generated `booking_reference`); create the CAPACITY child row; create one `booking_item` per guest seat item; write a `booking_history` entry; set guest booking status PROMOTED + `promoted_booking_id`. Return the new booking id.
- Tests: Service test: promoting a CONFIRMED guest booking creates a booking with matching item count and total; guest booking becomes PROMOTED; promoting twice or a non-CONFIRMED booking errors.
- Demo: Create+confirm a guest booking, promote it, show a real booking with N booking_items and the guest booking marked PROMOTED.

### Task 7: Authenticated promotion endpoint
- Objective: Expose promotion as an authenticated route so it runs after OAuth+payment and binds the real booking to the authenticated user.
- Guidance: Add `POST /guest-bookings/{id}/promote` in `apis/booking/src/routes/guest_booking.rs` using `Auth<CanCreateBooking>` (captures `user_id` from baggage). NOT under `/public`, so it requires the gateway-injected baggage/token. Add to OpenAPI docs.
- Tests: Integration test with a valid baggage header → promotion succeeds and returns the booking id; missing baggage → 401/insufficient permission.
- Demo: Through the gateway with an auth token, promote a confirmed guest booking and fetch the resulting real booking via `GET /api/booking/bookings/{id}?includes=items`.

## Notes / assumptions
- Guest routes rely on the `/public/...` prefix being exempt from `require_baggage_header` (verified).
- Promotion reuses the existing `CanCreateBooking` permission; a dedicated permission or guest role is an optional adjustment.
- Deep payment (Stripe/PayPal) wiring is out of scope here; payment is the authenticated step preceding promotion.
