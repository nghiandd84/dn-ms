# Payment Core Service API

Core payment processing, transaction orchestration, and integration with payment providers.

## Location

- API crate: `apis/payment-core/`
- Feature crates: `features/payments/core/{entities,model,repo,service,stream,migrations,remote}`
- Consul service name: `payment_core_service`

## Endpoints

### Payments
- `POST /payments` — Create payment (Auth: `CanCreatePayment`)
- `GET /payments` — Filter payments with pagination (Auth: `CanReadPayment`)
- `GET /payments/{id}` — Get payment by ID (Auth: `CanReadPayment`)
- `PATCH /payments/{id}` — Update payment (Auth: `CanUpdatePayment`). Publishes Kafka event on `status = "succeeded"`
- `DELETE /payments/{id}` — Delete payment (Auth: `CanDeletePayment`)

### Payment Attempts
- `POST /payment-attempts` — Create attempt
- `GET /payment-attempts` — Filter attempts
- `GET /payment-attempts/{id}` — Get by ID
- `PATCH /payment-attempts/{id}` — Update
- `DELETE /payment-attempts/{id}` — Delete

### Payment Methods
- `POST /payment-methods` — Create method
- `GET /payment-methods` — Filter methods
- `GET /payment-methods/{id}` — Get by ID
- `PATCH /payment-methods/{id}` — Update
- `DELETE /payment-methods/{id}` — Delete

### Payment Method Limits
- `POST /payment-method-limits` — Create limit
- `GET /payment-method-limits` — Filter limits
- `GET /payment-method-limits/{id}` — Get by ID
- `PATCH /payment-method-limits/{id}` — Update
- `DELETE /payment-method-limits/{id}` — Delete

## Payment Create Request

```json
{
  "user_id": "uuid",
  "amount": 2000,
  "currency": "usd",
  "provider_name": "stripe",
  "transaction_id": "order-001",
  "idempotency_key": "order-001-user-001",
  "gateway_transaction_id": "pi_xxx",
  "metadata": { "wallet_id": "uuid" }
}
```

The `metadata` field (JSON, nullable) carries context like `wallet_id` for downstream consumers.

## Kafka Producer

On `PATCH /payments/{id}` with `status = "succeeded"`:
- Publishes `PaymentCoreEventMessage::Succeeded` to `payment_core_topic`
- Message includes `payment_id`, `user_id`, `wallet_id` (from metadata), `amount`, `currency`
- Stream crate: `features/payments/core/stream/`
- Producer key: `"payment_core"`

## Kafka Consumer

Consumes `EventMessage` from event service topic (new/update events, currently placeholder handlers).

## Permissions

| Resource | Permissions |
|---|---|
| `PAYMENT_PAYMENT` | CREATE, READ, UPDATE, DELETE |
| `PAYMENT_METHOD` | CREATE, READ, UPDATE, DELETE |
| `PAYMENT_ATTEMPT` | CREATE, READ, UPDATE, DELETE |
| `PAYMENT_METHOD_LIMIT` | CREATE, READ, UPDATE, DELETE |

## Environment Variables

```
PAYMENT_CORE_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
PAYMENT_CORE_KAFKA_TOPIC=payment_core_topic
PAYMENT_CORE_CONSUMER_EVENT_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
PAYMENT_CORE_CONSUMER_EVENT_KAFKA_TOPIC=event_topic
DLQ_KAFKA_BOOTSTRAP_SERVERS=localhost:9092
DLQ_KAFKA_TOPIC=dlq_topic
```

## Integrations

- **Wallet Service** — via Kafka `payment_core_topic` (wallet credits on payment success)
- **Stripe Service** — calls payment-core via HTTP remote (`features/payments/core/remote/`)
- **Event Service** — consumes event stream
