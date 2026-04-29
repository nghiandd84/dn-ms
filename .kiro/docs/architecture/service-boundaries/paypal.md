# PayPal Service

Integrates with PayPal for payment processing via the Orders v2 API, webhooks, and reconciliation.

## Responsibilities
- Orchestrates end-to-end payment flow: create core payment → PayPal Order → buyer approval → capture → status update
- Handles PayPal webhook events
- Processes refunds via PayPal Captures API
- Persists PayPal-specific records (orders, refunds, webhook events, API logs)
- Maps PayPal events to internal payment status updates

## Dependencies
- **Payment Core Service** — called via HTTP remote (`features/payments/core/remote/`) for creating/updating payments and payment attempts
- **PayPal REST API v2** — called via `reqwest` for Order creation, capture, refunds, and OAuth2 token exchange
- **Consul** — service discovery for payment-core remote calls

## Data Flow

```
Frontend                    PayPal Service                 Payment Core          PayPal API
   │                             │                              │                    │
   │  POST /flow/initiate │                              │                    │
   │────────────────────────────>│  create_payment (remote)     │                    │
   │                             │─────────────────────────────>│                    │
   │                             │<─────────────────────────────│                    │
   │                             │  POST /v2/checkout/orders    │                    │
   │                             │──────────────────────────────────────────────────>│
   │                             │<──────────────────────────────────────────────────│
   │                             │  update_payment (remote)     │                    │
   │                             │─────────────────────────────>│                    │
   │   { approval_url }          │                              │                    │
   │<────────────────────────────│                              │                    │
   │                             │                              │                    │
   │  (User approves on PayPal)  │                              │                    │
   │                             │                              │                    │
   │  POST /flow/capture  │                              │                    │
   │────────────────────────────>│  POST /v2/.../capture        │                    │
   │                             │──────────────────────────────────────────────────>│
   │                             │<──────────────────────────────────────────────────│
   │                             │  update_payment (remote)     │                    │
   │                             │─────────────────────────────>│  Kafka: succeeded  │
   │   { capture_id, status }    │                              │─────> Wallet       │
   │<────────────────────────────│                              │                    │
```

## Error Handling Strategy
- **PayPal API failure during initiate**: core payment marked as `"failed"`, failed attempt logged, error returned
- **Post-PayPal DB failures during initiate**: logged but `approval_url` still returned — webhook reconciles later
- **Capture failure**: PayPal error returned
- **Refund failure**: PayPal error returned, no partial state changes

## Endpoints
- `POST /flow/initiate` — Initiate payment (creates core payment + PayPal Order)
- `POST /flow/capture` — Capture after buyer approval
- `POST /flow/webhook` — PayPal webhook handler (public)
- `POST /flow/refund` — Refund a captured payment
- CRUD for: `/orders`, `/refunds`, `/webhook-events`, `/api-logs`
