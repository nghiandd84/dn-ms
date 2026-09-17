# Implementation Plan — SvelteKit Event Booking App (separate codebase)

## Problem Statement
Build a SvelteKit web app where guests browse events, select seats, create and confirm a guest booking without logging in, then authenticate via OAuth to pay — after which the booking is promoted to a real booking. All API calls go through the gateway at `http://localhost:6001/api` with an `Origin` header.

## Requirements
- SvelteKit + TypeScript, separate codebase.
- Talk to backend through the API Gateway; CORS allows `localhost` origins (dev port 5173).
- Guest browse/create/confirm require NO token.
- Payment requires the OAuth authorization-code flow via `/api/auth`.
- One booking item per selected seat.
- Screens: Events list → Event detail + seat select/book → My booking (confirm) → Payment (OAuth) → confirmation.

## Backend endpoints used
- Browse: `GET /api/event/public/events?status=li|UPCOMING&order_name=event_date`, `GET /api/event/public/events/{id}` (public, no baggage).
- Guest booking: `POST /api/booking/public/guest-bookings`, `GET /api/booking/public/guest-bookings/{id}`, `POST /api/booking/public/guest-bookings/{id}/confirm` (public).
- OAuth: `POST /api/auth/public/requests/code`, `POST /api/auth/requests/login`, `POST /api/auth/public/tokens/oauth`.
- Promotion (authenticated): `POST /api/booking/guest-bookings/{id}/promote` with `Authorization: Bearer <token>`.
- Result: `GET /api/booking/bookings/{id}?includes=items`.

(Depends on the backend guest-booking API plan being implemented first — see `.kiro/plans/api-guest-booking.md`.)

## Task Breakdown

### Task 1: Scaffold + Events list & Event detail (browse)
- Objective: Scaffold the SvelteKit app and implement the events list and detail screens against the public event API.
- Guidance: Scaffold SvelteKit + TypeScript. Add an API client with base `http://localhost:6001/api` that sends the `Origin` header. Implement `/` (events list → `GET /api/event/public/events?status=li|UPCOMING&order_name=event_date`) and `/events/[id]` (`GET /api/event/public/events/{id}`) with a seat-selection UI (choose seats up to `total_seats`).
- Tests: Vitest component/route tests with mocked fetch for list rendering and detail seat selection.
- Demo: Run the dev server; browse the events list and open an event detail with selectable seats (live data via gateway).

### Task 2: Guest booking create & confirm
- Objective: Implement guest booking creation and the confirm screen.
- Guidance: On "Book", `POST /api/booking/public/guest-bookings` with event_id, guest info, and one seat entry per selected seat; store the returned guest booking id (store/localStorage). Add `/booking/[id]` "My booking" screen showing status and a Confirm action (`POST .../public/guest-bookings/{id}/confirm`).
- Tests: Vitest tests with mocked API for create payload shape (one item per seat) and confirm transition.
- Demo: Select seats, create a guest booking (PENDING), view it, and confirm it (CONFIRMED) through the gateway.

### Task 3: OAuth payment & promotion (wire-up)
- Objective: Implement the OAuth authorization-code flow for payment, then trigger promotion.
- Guidance: After confirm, "Pay" starts OAuth: `POST /api/auth/public/requests/code` → login (`/api/auth/requests/login`) → exchange code at `/api/auth/public/tokens/oauth` for an access token. Add a `redirect_uri` route to capture the code. With the token, perform the (mock or real) payment, then call the authenticated `POST /api/booking/guest-bookings/{id}/promote` with `Authorization: Bearer <token>`. On success, show the confirmation with the real booking id and fetch `GET /api/booking/bookings/{id}?includes=items`.
- Tests: Vitest tests mocking token exchange and promotion; verify the Authorization header is attached only to authenticated requests.
- Demo: Full happy path — browse → select seats → guest booking → confirm → OAuth login → pay → promotion → view the real booking with one item per seat.

## Notes
- Dev origin should match the gateway CORS allowlist (`localhost`); Vite default is `http://localhost:5173`.
- The OAuth `client_id`, `scopes`, and `redirect_uri` must match a registered auth client; reuse values from `apis/auth/test.rest` / `apps/gateway/test.rest` during development.
