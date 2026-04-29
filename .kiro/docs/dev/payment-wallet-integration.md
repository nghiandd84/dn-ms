# Payment-to-Wallet Integration

End-to-end flow from Stripe payment success to wallet balance credit via Kafka.

## Architecture

```
Client                    Stripe Service              Payment-Core             Wallet
  |                            |                           |                     |
  | POST /flow/initiate |                           |                     |
  | { amount, currency,        |                           |                     |
  |   metadata: {wallet_id} }  |                           |                     |
  |                            |-- create_payment -------->|                     |
  |                            |   (metadata persisted)    |                     |
  |                            |                           |                     |
  |        ... Stripe confirms payment via webhook ...     |                     |
  |                            |                           |                     |
  |                            |-- update status=succeeded |                     |
  |                            |                           |-- Kafka event ----->|
  |                            |                           |   payment_core_topic|
  |                            |                           |                     |
  |                            |                           |   Consumer:         |
  |                            |                           |   1. Check wallet_id|
  |                            |                           |   2. Idempotency    |
  |                            |                           |   3. Credit balance |
  |                            |                           |   4. Create tx      |
```

## Kafka Event

Topic: `payment_core_topic`

When a payment is updated to `status = "succeeded"`, payment-core publishes:

```json
{
  "event_type": "succeeded",
  "message": {
    "payment_id": "uuid",
    "user_id": "uuid",
    "wallet_id": "uuid or null",
    "amount": 2000,
    "currency": "usd"
  }
}
```

The `wallet_id` is extracted from the payment's `metadata` JSON field (`metadata.wallet_id`).

## Payment Metadata

The `payments` table has a `metadata` column (JSON, nullable). Clients pass `wallet_id` when initiating a payment:

```json
{
  "amount": 2000,
  "currency": "usd",
  "metadata": { "wallet_id": "wallet-uuid" }
}
```

This metadata flows through: client → Stripe service → payment-core (persisted) → Kafka event → wallet consumer.

## Wallet Consumer

Location: `apis/wallet/src/consumers/payment_core_consumer/handler.rs`

Handles `PaymentCoreEventMessage::Succeeded`:

1. **Skip if no wallet_id** — not all payments target a wallet
2. **Idempotency check** — queries transactions by `reference_id = "payment:{payment_id}"`, skips if exists
3. **Credit wallet** — calls `WalletService::credit_wallet(wallet_id, amount)` with optimistic locking retry
4. **Create transaction** — creates a `DEPOSIT` transaction with `reference_id` for idempotency tracking

Failed messages are sent to the DLQ topic.

### Env vars (wallet service)

```
WALLET_CONSUMER_PAYMENT_CORE_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
WALLET_CONSUMER_PAYMENT_CORE_KAFKA_TOPIC=payment_core_topic
DLQ_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
DLQ_KAFKA_TOPIC=dlq_topic
```

## Wallet Balance Operations

Location: `features/wallet/service/src/wallet.rs`

### credit_wallet(wallet_id, amount)

1. Read wallet (get current balance + version)
2. Compute `new_balance = current + amount`
3. Update wallet with `new_balance` and `version + 1`
4. On failure (version mismatch), retry up to 3 times with backoff

### debit_wallet(wallet_id, amount)

Same pattern but checks `current_balance >= amount` before deducting.

### Layer separation

- **Repo** (`WalletMutation`) — pure CRUD: `create_wallet`, `update_wallet`, `delete_wallet`
- **Service** (`WalletService`) — owns business logic: read → validate → compute → update → retry

The `version` column on `wallets` table enables optimistic locking. The `WalletForUpdateDto` includes `version` so the service can increment it on each balance change.

## Stream Crate

Location: `features/payments/core/stream/`

```rust
pub enum PaymentCoreEventMessage {
    Succeeded { message: PaymentSucceededMessage },
}

pub struct PaymentSucceededMessage {
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
}

pub const PRODUCER_KEY: &str = "payment_core";
```

### Env vars (payment-core service)

```
PAYMENT_CORE_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
PAYMENT_CORE_KAFKA_TOPIC=payment_core_topic
```
