# PayPal Service API

Integrates with PayPal for payment processing via the Orders v2 API, webhooks, and reconciliation.

## Location
- API crate: `apis/payments/paypal/`
- Feature crates: `features/payments/paypal/{entities,model,repo,service,stream,migrations}`
- Core payment remote: `features/payments/core/remote/`
- Port: `5181` (env: `PAYMENT_PAYPAL_PORT`)
- Consul service name: `payment_core_service` (for remote calls to payment-core)

## Payment Flow Endpoints

### POST /flow/initiate
Initiates a payment: creates a core payment record, calls PayPal to create an Order, and returns the `approval_url` for the buyer to approve on PayPal.

**Auth:** `Auth<CanCreateOrder>`

**Request:**
```json
{
  "user_id": "uuid",
  "amount": 2000,
  "currency": "usd",
  "idempotency_key": "order-001-user-001",
  "metadata": { "wallet_id": "uuid" }
}
```

**Response:**
```json
{
  "payment_id": "uuid",
  "paypal_order_id": "5O190127TN364715T",
  "approval_url": "https://www.sandbox.paypal.com/checkoutnow?token=5O190127TN364715T"
}
```

**Error handling:**
- If PayPal API fails or returns 4xx/5xx → core payment marked `"failed"`, failed attempt logged, error returned.
- If post-PayPal DB updates fail → logged as errors but `approval_url` still returned. Webhook reconciles.

### POST /flow/capture
Captures a payment after the buyer approves on PayPal. Looks up the local order record, calls PayPal capture API, and updates core payment status.

**Auth:** `Auth<CanCreateOrder>`

**Request:**
```json
{
  "paypal_order_id": "5O190127TN364715T"
}
```

**Response:**
```json
{
  "payment_id": "uuid",
  "capture_id": "2GG279541U471931P",
  "status": "COMPLETED"
}
```

When `status` is `"COMPLETED"`, core payment is updated to `"succeeded"` → triggers Kafka event → wallet credit.

### POST /flow/webhook
Receives PayPal webhook events. Persists the event and updates payment/order status.

**Auth:** `PublicAccess` (called by PayPal)

**Handled event types:**
- `CHECKOUT.ORDER.APPROVED` → updates local order status to `"APPROVED"`
- `PAYMENT.CAPTURE.COMPLETED` → updates order to `"COMPLETED"`, core payment to `"succeeded"`
- `PAYMENT.CAPTURE.DENIED` → updates core payment to `"failed"`
- `PAYMENT.CAPTURE.REFUNDED` → logged

### POST /flow/refund
Refunds a captured payment via PayPal API.

**Auth:** `Auth<CanCreateOrder>`

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
  "paypal_refund_id": "1JU08902781691411",
  "status": "COMPLETED"
}
```

## CRUD Endpoints

### PayPal Orders
- `POST /orders` — Create record
- `GET /orders` — List/filter
- `GET /orders/{id}` — Get by ID
- `PATCH /orders/{id}` — Update
- `DELETE /orders/{id}` — Delete

### PayPal Refunds
- `POST /refunds` — Create record
- `GET /refunds` — List/filter
- `GET /refunds/{id}` — Get by ID
- `PATCH /refunds/{id}` — Update
- `DELETE /refunds/{id}` — Delete

### PayPal Webhook Events
- `POST /webhook-events` — Create record
- `GET /webhook-events` — List/filter
- `GET /webhook-events/{id}` — Get by ID
- `PATCH /webhook-events/{id}` — Update
- `DELETE /webhook-events/{id}` — Delete

### PayPal API Logs
- `POST /api-logs` — Create record
- `GET /api-logs` — List/filter
- `GET /api-logs/{id}` — Get by ID
- `PATCH /api-logs/{id}` — Update
- `DELETE /api-logs/{id}` — Delete

## Permissions

| Resource | Constant |
|----------|----------|
| `PAYPAL_ORDER` | CREATE, READ, UPDATE, DELETE |
| `PAYPAL_REFUND` | CREATE, READ, UPDATE, DELETE |
| `PAYPAL_WEBHOOK_EVENT` | CREATE, READ, UPDATE, DELETE |
| `PAYPAL_API_LOG` | CREATE, READ, UPDATE, DELETE |

## Integrations
- **Payment Core Service** — via `features/payments/core/remote/` (HTTP calls through Consul)
- **PayPal REST API v2** — via `reqwest` HTTP client (Orders, Captures, Refunds)

## Environment Variables
- `PAYPAL_CLIENT_ID` — PayPal app client ID
- `PAYPAL_CLIENT_SECRET` — PayPal app client secret
- `PAYPAL_API_BASE` — API base URL (default: `https://api-m.sandbox.paypal.com`)
- `PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_GET_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT` — `/payments`
- `PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT` — `/payment-attempts`
