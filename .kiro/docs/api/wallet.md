# Wallet Service API

Manages user wallets, balances, transactions, top-ups, P2P transfers, and withdrawals.

## Location

- API crate: `apis/wallet/`
- Feature crates: `features/wallet/{entities,model,repo,service,stream,migrations}`

## Endpoints

### Wallets
- `POST /wallets` — Create wallet (Auth: `CanCreateWallet`)
- `GET /wallets` — Filter wallets (Auth: `CanReadWallet`)
- `GET /wallets/{id}` — Get by ID (Auth: `CanReadWallet`)
- `GET /wallets/user/{user_id}` — Get by user (Auth: `CanReadWallet`)
- `PATCH /wallets/{id}` — Update (Auth: `CanUpdateWallet`)
- `DELETE /wallets/{id}` — Delete (Auth: `CanDeleteWallet`)
- `GET /wallets/{id}/transactions` — Wallet transactions (Auth: `CanReadWallet`)
- `GET /wallets/{id}/top-ups` — Wallet top-ups (Auth: `CanReadWallet`)

### Transactions
- `POST /transactions` — Create (Auth: `CanCreateTransaction`)
- `GET /transactions` — Filter (Auth: `CanReadTransaction`)
- `GET /transactions/{id}` — Get by ID (Auth: `CanReadTransaction`)
- `PATCH /transactions/{id}` — Update
- `DELETE /transactions/{id}` — Delete

### Top-Ups, P2P Transfers, Withdrawals, Idempotency Keys
Standard CRUD endpoints with auth guards and idempotency key support.

## Balance Operations

### credit_wallet(wallet_id, amount)
Service-level operation with optimistic locking:
1. Read wallet → get balance + version
2. Compute new balance
3. Update with `version + 1`
4. Retry up to 3 times on version conflict

### debit_wallet(wallet_id, amount)
Same pattern, with insufficient balance check before deducting.

The `version` column on `wallets` table enables optimistic locking. Business logic lives in `WalletService`, repo layer is pure CRUD.

## Kafka Consumer

Consumes `PaymentCoreEventMessage` from `payment_core_topic`:
- On `Succeeded` event with `wallet_id`: credits wallet balance and creates `DEPOSIT` transaction
- Idempotency via `reference_id = "payment:{payment_id}"` on transactions
- Failed messages sent to DLQ

Consumer location: `apis/wallet/src/consumers/payment_core_consumer/`

## Permissions

| Resource | Permissions |
|---|---|
| `WALLET_WALLET` | CREATE, READ, UPDATE, DELETE |
| `WALLET_TRANSACTION` | CREATE, READ |
| `WALLET_P2P_TRANSFER` | CREATE, READ |
| `WALLET_TOP_UP` | CREATE, READ |
| `WALLET_WITHDRAWAL` | CREATE, READ, UPDATE |
| `WALLET_IDEMPOTENCY` | CREATE, READ, UPDATE |

## Environment Variables

```
WALLET_CONSUMER_PAYMENT_CORE_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
WALLET_CONSUMER_PAYMENT_CORE_KAFKA_TOPIC=payment_core_topic
DLQ_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
DLQ_KAFKA_TOPIC=dlq_topic
```

## Integrations

- **Payment Core Service** — consumes Kafka events for wallet credits on payment success
- **Idempotency middleware** — tracks all requests via SHA-256 hash of method|uri|body
