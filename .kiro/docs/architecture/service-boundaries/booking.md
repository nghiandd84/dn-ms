# Booking Service

Generalized multi-type booking service. A single core model supports many
booking domains via a `booking_mode` discriminator, rather than being tied to
events.

## Responsibilities
- Create, read, update, and delete bookings across many domains (events, hotel
  rooms, car/equipment rentals, appointments, workshops, waitlists, on-demand
  delivery, permits, ...).
- Classify each booking by `booking_type` (domain label) and `booking_mode`
  (behavior/shape).
- Persist mode-specific data in dedicated child tables and enforce mode-specific
  availability within a single transaction.
- Manage booking line items (`booking_items`) — the concrete units (seats,
  tables, rooms, cars, slots) attached to a booking.
- Maintain an append-only lifecycle history (`booking_history`) recording
  creation, status changes, payment updates, and cancellations, written
  transactionally with each change.

## Booking Modes
- `WINDOW` — time-range reservation of a unit (overlap guard).
- `CAPACITY` — finite slots in a container (counter guard via `metadata.capacity_limit`).
- `RECURRENCE` — repeating series / validity-period entitlement.
- `APPROVAL` — request-to-book / host-approval workflow.
- `QUEUE` — waitlist with auto-assigned position.
- `DISPATCH` — on-demand fulfillment matched to a provider.

## Boundaries
- Owns the `bookings`, `booking_items`, and six mode child tables.
- Does **not** own the booked resources — those live in other services and are
  referenced polymorphically via `resource_type` + `resource_id` (and container
  ids for CAPACITY). The booking service does not currently validate that the
  referenced resource exists.
- Payment state is tracked on the booking (`payment_id`, `payment_status`) but
  payment processing itself is owned by the payment/wallet services.

## Integrations
- Payment / Wallet — for settling and reflecting payment status.
- Resource-owning services (event, inventory, etc.) referenced by
  `resource_type` / `resource_id`.
- Notification — for reminders / approval / queue-offer events (future).

## Endpoints
- `/bookings`, `/bookings/{id}`
- `/bookings/{id}/history`
- `/booking-items`, `/booking-items/{id}`

## Delete policy
`DELETE /bookings/{id}` is a **soft-delete**: it sets `status = CANCELLED` and
appends a `CANCELLED` history event rather than physically removing the row, so
the booking and its history are retained for audit.

## Known limitations / future work
- Availability guards are transaction-scoped, not strictly race-proof; DB-level
  constraints (`EXCLUDE USING gist` for WINDOW) or serializable isolation for
  CAPACITY are the recommended hardening.
- APPROVAL / QUEUE / DISPATCH support creation only; their state-machine
  transitions (approve/decline, offer/accept, assign/complete) and event
  emission are not yet implemented.
