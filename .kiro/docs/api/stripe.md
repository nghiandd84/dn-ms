# Stripe Service API

Integrates with Stripe for payment processing, webhooks, and reconciliation.

## Location
- API crate: `apis/paypments/stripe/`
- Feature crates: `features/payments/stripe/{entities,model,repo,service,stream,migrations}`
- Core payment remote: `features/payments/core/remote/`
- Port: `5121` (env: `PAYMENT_STRIPE_PORT`)
- Consul service name: `payment_core_service` (for remote calls to payment-core)

## Payment Flow Endpoints

### POST /stripe/flow/initiate
Initiates an end-to-end payment: creates a core payment record, calls Stripe to create a PaymentIntent, and returns the `client_secret` for frontend Stripe.js confirmation.

**Auth:** `Auth<CanCreatePaymentIntent>`

**Request:**
```json
{
  "user_id": "uuid",
  "amount": 2000,
  "currency": "usd",
  "idempotency_key": "order-001-user-001",
  "metadata": { "order_id": "order-001" }
}
```

**Response:**
```json
{
  "payment_id": "uuid",
  "stripe_payment_intent_id": "pi_xxx",
  "client_secret": "pi_xxx_secret_xxx"
}
```

**Error handling:**
- If Stripe API fails → core payment is marked `"failed"`, a failed payment attempt is logged, error returned to caller.
- If post-Stripe DB updates fail (update core payment, persist stripe PI record) → logged as errors but `client_secret` is still returned since the Stripe PaymentIntent exists. Webhook will reconcile.

### POST /stripe/flow/webhook
Receives Stripe webhook events. Verifies signature, persists the event, and updates payment + PI status.

**Auth:** `PublicAccess` (called by Stripe)

**Headers:** `stripe-signature` (required)

**Handled event types:**
- `payment_intent.succeeded` → status `"succeeded"`
- `payment_intent.payment_failed` → status `"failed"`
- `payment_intent.canceled` → status `"canceled"`
- `payment_intent.processing` → status `"processing"`

### POST /stripe/flow/refund
Creates a refund via Stripe API and persists records.

**Auth:** `Auth<CanCreatePaymentIntent>`

**Request:**
```json
{
  "payment_id": "uuid",
  "amount": 1000,
  "reason": "requested_by_customer"
}
```
Omit `amount` for full refund.

**Response:**
```json
{
  "refund_id": "uuid",
  "stripe_refund_id": "re_xxx",
  "status": "succeeded"
}
```

## CRUD Endpoints

### Stripe Payment Intents
- `POST /stripe/payment-intents` — Create record
- `GET /stripe/payment-intents` — List/filter
- `GET /stripe/payment-intents/{id}` — Get by ID
- `PATCH /stripe/payment-intents/{id}` — Update
- `DELETE /stripe/payment-intents/{id}` — Delete

### Stripe Refunds
- `POST /stripe/refunds` — Create record
- `GET /stripe/refunds` — List/filter
- `GET /stripe/refunds/{id}` — Get by ID
- `PATCH /stripe/refunds/{id}` — Update
- `DELETE /stripe/refunds/{id}` — Delete

### Stripe Webhook Events
- `POST /stripe/webhook-events` — Create record
- `GET /stripe/webhook-events` — List/filter
- `GET /stripe/webhook-events/{id}` — Get by ID
- `PATCH /stripe/webhook-events/{id}` — Update
- `DELETE /stripe/webhook-events/{id}` — Delete

### Stripe API Logs
- `POST /stripe/api-logs` — Create record
- `GET /stripe/api-logs` — List/filter
- `GET /stripe/api-logs/{id}` — Get by ID
- `PATCH /stripe/api-logs/{id}` — Update
- `DELETE /stripe/api-logs/{id}` — Delete

## Permissions

| Resource | Constant |
|----------|----------|
| `STRIPE_PAYMENT_INTENT` | CREATE, READ, UPDATE, DELETE |
| `STRIPE_REFUND` | CREATE, READ, UPDATE, DELETE |
| `STRIPE_WEBHOOK_EVENT` | CREATE, READ, UPDATE, DELETE |
| `STRIPE_API_LOG` | CREATE, READ, UPDATE, DELETE |

## Integrations
- **Payment Core Service** — via `features/payments/core/remote/` (HTTP calls through Consul)
- **Stripe API** — via `async-stripe` crate v0.41

## Environment Variables
- `STRIPE_SECRET_KEY` — Stripe API secret key
- `STRIPE_WEBHOOK_SECRET` — Webhook endpoint signing secret
- `PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_GET_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT` — `/payment-attempts`
