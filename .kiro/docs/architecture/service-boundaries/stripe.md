# Stripe Service

Integrates with Stripe for payment processing, webhooks, and reconciliation.

## Responsibilities
- Orchestrates end-to-end payment flow: create core payment → Stripe PaymentIntent → webhook → status update
- Handles Stripe webhook events with signature verification
- Processes refunds via Stripe API
- Persists Stripe-specific records (payment intents, refunds, webhook events, API logs)
- Maps Stripe events to internal payment status updates

## Dependencies
- **Payment Core Service** — called via HTTP remote (`features/payments/core/remote/`) for creating/updating payments and payment attempts
- **Stripe API** — called via `async-stripe` crate for PaymentIntent creation, refunds, webhook verification
- **Consul** — service discovery for payment-core remote calls

## Data Flow

```
Frontend                    Stripe Service                 Payment Core          Stripe API
   │                             │                              │                    │
   │  POST /stripe/flow/initiate │                              │                    │
   │────────────────────────────>│  create_payment (remote)     │                    │
   │                             │─────────────────────────────>│                    │
   │                             │<─────────────────────────────│                    │
   │                             │  CreatePaymentIntent         │                    │
   │                             │──────────────────────────────────────────────────>│
   │                             │<──────────────────────────────────────────────────│
   │                             │  update_payment (remote)     │                    │
   │                             │─────────────────────────────>│                    │
   │   { client_secret }         │                              │                    │
   │<────────────────────────────│                              │                    │
   │                             │                              │                    │
   │  (Stripe.js confirms)       │                              │                    │
   │                             │  POST /stripe/flow/webhook   │                    │
   │                             │<──────────────────────────────────────────────────│
   │                             │  update statuses (remote)    │                    │
   │                             │─────────────────────────────>│                    │
```

## Error Handling Strategy
- **Stripe API failure during initiate**: core payment marked as `"failed"`, failed attempt logged, error returned
- **Post-Stripe DB failures during initiate**: logged but `client_secret` still returned — webhook reconciles later
- **Webhook signature failure**: rejected with error, event not persisted
- **Refund failure**: Stripe error returned, no partial state changes

## Endpoints
- `POST /stripe/flow/initiate` — Initiate payment (creates core payment + Stripe PI)
- `POST /stripe/flow/webhook` — Stripe webhook handler (public)
- `POST /stripe/flow/refund` — Refund a payment
- CRUD for: `/stripe/payment-intents`, `/stripe/refunds`, `/stripe/webhook-events`, `/stripe/api-logs`
