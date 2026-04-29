# PayPal Payment Flow

End-to-end payment orchestration using PayPal as the payment provider.

## Location
- Flow service: `features/payments/paypal/service/src/payment_flow.rs`
- Flow models: `features/payments/paypal/model/src/payment_flow.rs`
- Flow routes: `apis/payments/paypal/src/routes/payment_flow.rs`
- Core remote: `features/payments/core/remote/src/`

## Architecture

The `PaymentFlowService` orchestrates calls across:
1. **Payment Core** (via `features/payments/core/remote/`) — HTTP remote calls through Consul
2. **PayPal REST API v2** (via `reqwest`) — direct HTTP calls with OAuth2 Bearer tokens
3. **PayPal DB records** (via local paypal service/repo) — direct DB access

This follows the same remote service pattern as Stripe: the PayPal service does NOT depend on `features-payments-core-service` directly. It calls the payment-core API over HTTP using `PaymentRemoteService` and `PaymentAttemptRemoteService`.

## Key Difference from Stripe

PayPal uses a **redirect-based approval flow** instead of Stripe's embedded client-side confirmation:

| Aspect | Stripe | PayPal |
|--------|--------|--------|
| Initiate returns | `client_secret` (for Stripe.js) | `approval_url` (redirect to PayPal) |
| Confirmation | Client-side via Stripe.js | Server-side capture after redirect back |
| Extra step | None | `POST /flow/capture` required |
| SDK | `async-stripe` crate | `reqwest` + PayPal REST API v2 |
| Auth | API key | OAuth2 client credentials |

## PayPal Client Setup

The `PaymentsPaypalAppState` holds a `reqwest::Client` and PayPal credentials:

```rust
// features/payments/paypal/model/src/state.rs
#[derive(Clone)]
pub struct PaymentsPaypalAppState {
    pub http_client: reqwest::Client,
    pub client_id: String,
    pub client_secret: String,
    pub api_base: String,
}
```

OAuth2 access tokens are obtained per-request via `POST /v1/oauth2/token` with Basic auth (client_id:client_secret).

## Initiate Payment Flow

`PaymentFlowService::initiate_payment(state, baggage, req)`

| Step | Action | On Failure |
|------|--------|------------|
| 1 | Create core payment via `PaymentRemoteService::create_payment()` | Return error |
| 2 | Get OAuth2 access token from PayPal | Return error |
| 3 | Call `POST /v2/checkout/orders` with `intent: CAPTURE` | Mark payment `"failed"`, log failed attempt, return error |
| 4 | Update core payment with `gateway_transaction_id` (order ID) via remote | Log error, continue |
| 5 | Persist `paypal_orders` record | Log error, continue |
| 6 | Log payment attempt via `PaymentAttemptRemoteService` | Fire and forget |

Returns `{ payment_id, paypal_order_id, approval_url }`.

The frontend redirects the user to `approval_url`. After approval, PayPal redirects back to the app.

## Capture Flow

`PaymentFlowService::capture_payment(state, baggage, req)`

Called after the buyer approves the payment on PayPal:

| Step | Action |
|------|--------|
| 1 | Get OAuth2 access token |
| 2 | Call `POST /v2/checkout/orders/{order_id}/capture` |
| 3 | Find local `paypal_orders` record by `paypal_order_id` |
| 4 | Update local order with `capture_id` and status |
| 5 | Update core payment status (`"succeeded"` if COMPLETED) via remote |

When core payment is updated to `"succeeded"`, payment-core publishes a Kafka event → wallet consumer credits the wallet.

## Webhook Flow

`PaymentFlowService::process_webhook(payload, baggage)`

1. Parse JSON payload to extract `id`, `event_type`, and `resource`
2. Persist `paypal_webhook_events` record
3. Handle event types:
   - `CHECKOUT.ORDER.APPROVED` → update local order status
   - `PAYMENT.CAPTURE.COMPLETED` → update order + core payment to `"succeeded"`
   - `PAYMENT.CAPTURE.DENIED` → update core payment to `"failed"`

## Refund Flow

`PaymentFlowService::refund_payment(state, baggage, req)`

1. Look up core payment via `PaymentRemoteService::get_payment_by_id()` to get `gateway_transaction_id`
2. Find local order by PayPal order ID to get `capture_id`
3. Get OAuth2 access token
4. Call `POST /v2/payments/captures/{capture_id}/refund`
5. Persist `paypal_refunds` record
6. Update core payment status to `"refunded"` via remote

## Amount Handling

PayPal expects amounts as decimal strings (e.g., `"19.99"`), while the system stores amounts in minor units (cents). The `format_amount` helper converts:

```rust
fn format_amount(amount: i64) -> String {
    format!("{}.{:02}", amount / 100, amount % 100)
}
```

## Consul Discovery

Same as Stripe — `PaymentRemoteService::update_remote(&consul_client)` is called every 30 seconds in `app.rs` custom_handler.

### Required Environment Variables
```
PAYPAL_CLIENT_ID=your-client-id
PAYPAL_CLIENT_SECRET=your-client-secret
PAYPAL_API_BASE=https://api-m.sandbox.paypal.com
PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_GET_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT=/payment-attempts
```
