# Stripe Payment Flow

End-to-end payment orchestration using Stripe as the payment provider.

## Location
- Flow service: `features/payments/stripe/service/src/payment_flow.rs`
- Flow models: `features/payments/stripe/model/src/payment_flow.rs`
- Flow routes: `apis/paypments/stripe/src/routes/payment_flow.rs`
- Core remote: `features/payments/core/remote/src/`

## Architecture

The `PaymentFlowService` orchestrates calls across:
1. **Payment Core** (via `features/payments/core/remote/`) — HTTP remote calls through Consul
2. **Stripe API** (via `async-stripe` crate) — direct API calls
3. **Stripe DB records** (via local stripe service/repo) — direct DB access

This follows the remote service pattern: the stripe service does NOT depend on `features-payments-core-service` directly. It calls the payment-core API over HTTP using `PaymentRemoteService` and `PaymentAttemptRemoteService`.

## Stripe Client Setup

The `stripe::Client` is stored in `PaymentsStripeAppState` and created from `STRIPE_SECRET_KEY` at startup:

```rust
// features/payments/stripe/model/src/state.rs
#[derive(Clone)]
pub struct PaymentsStripeAppState {
    pub stripe_client: stripe::Client,
}

// apis/paypments/stripe/src/app.rs
let stripe_client = stripe::Client::new(std::env::var("STRIPE_SECRET_KEY")?);
my_app.start_app(Some(PaymentsStripeAppState { stripe_client })).await?;
```

Accessed in route handlers via:
```rust
let client = &app_state.get_state().unwrap().stripe_client;
```

## Initiate Payment Flow

`PaymentFlowService::initiate_payment(client, req)`

| Step | Action | On Failure |
|------|--------|------------|
| 1 | Create core payment via `PaymentRemoteService::create_payment()` | Return error |
| 2 | Call `stripe::PaymentIntent::create()` | Mark payment `"failed"`, log failed attempt, return error |
| 3 | Update core payment with `gateway_transaction_id` via remote | Log error, continue (webhook reconciles) |
| 4 | Persist `stripe_payment_intents` record | Log error, continue |
| 5 | Log payment attempt via `PaymentAttemptRemoteService` | Fire and forget |

Returns `{ payment_id, stripe_payment_intent_id, client_secret }`.

## Webhook Flow

`PaymentFlowService::process_webhook(payload, signature, webhook_secret)`

1. Verify signature via `stripe::Webhook::construct_event()`
2. Parse raw JSON payload to extract `data.object.id` (Stripe PI id)
3. Persist `stripe_webhook_events` record
4. Look up local `stripe_payment_intents` record by `stripe_payment_intent_id`
5. Update both stripe PI status and core payment status (via remote)

## Refund Flow

`PaymentFlowService::refund_payment(client, req)`

1. Look up core payment via `PaymentRemoteService::get_payment_by_id()` to get `gateway_transaction_id`
2. Call `stripe::Refund::create()` with the PaymentIntent ID
3. Persist `stripe_refunds` record
4. Update core payment status to `"refunded"` via remote

## Core Payment Remote

`features/payments/core/remote/` provides HTTP clients for the payment-core API:

```rust
#[derive(Debug, RemoteService)]
#[remote(name(payment_core_service))]
pub struct PaymentRemoteService {}
```

Methods:
- `create_payment(req) -> Result<Uuid, String>`
- `get_payment_by_id(id) -> Result<PaymentData, String>`
- `update_payment(id, req) -> Result<bool, String>`

```rust
#[derive(Debug, RemoteService)]
#[remote(name(payment_core_service))]
pub struct PaymentAttemptRemoteService {}
```

Methods:
- `create_payment_attempt(req) -> Result<Uuid, String>`

### Required Environment Variables
```
PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_GET_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_UPDATE_PAYMENT=/payments
PAYMENT_CORE_ENDPOINT_CREATE_PAYMENT_ATTEMPT=/payment-attempts
```

### Consul Discovery
`PaymentRemoteService::update_remote(&consul_client)` must be called periodically (done in `app.rs` custom_handler alongside `PermissionService::update_remote`).
