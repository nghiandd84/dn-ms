# Wallet Microservice

A complete, production-ready wallet management microservice for the dn-ms monorepo. Manages user wallets, transactions, balance tracking, and supports multiple currencies.

## Architecture

The wallet microservice follows a **4-layer hexagonal architecture**:

```
HTTP Request
    ↓
API Routes (swagger endpoints)
    ↓
Service Layer (business logic)
    ↓
Repository Layer (data access)
    ↓
Entities Layer (database models)
    ↓
PostgreSQL Database
```

## Project Structure

```
features/wallet/                 # Business logic and data models
├── entities/                     # Database entities (Sea-orm models)
│   ├── wallet.rs               # Wallet database model
│   └── transaction.rs          # Transaction database model
├── migrations/                  # Database migrations
│   └── m20260101_000001_...   # Create wallet & transaction tables
├── model/                        # API DTOs and request/response models
│   ├── wallet.rs               # Wallet DTO & requests
│   ├── transaction.rs          # Transaction DTO & requests
│   └── state.rs                # App state definitions
├── repo/                         # Repository pattern (data access)
│   ├── wallet/                 # Wallet queries & mutations
│   └── transaction/            # Transaction queries & mutations
├── service/                      # Business logic
│   ├── wallet.rs               # Wallet service methods
│   └── transaction.rs          # Transaction service methods
└── stream/                       # Event & message definitions
    ├── wallet_event.rs         # Wallet events
    └── transaction_event.rs    # Transaction events

apis/wallet/                      # REST API implementation
├── src/
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # App initialization & routing
│   ├── doc.rs                  # OpenAPI documentation
│   └── routes/
│       ├── wallet.rs           # Wallet endpoints
│       └── transaction.rs      # Transaction endpoints
└── Cargo.toml
```

## Features

### Wallet Management
- ✅ Create user wallets
- ✅ Get wallet details by wallet ID or user ID
- ✅ Update wallet (balance, currency, status)
- ✅ Delete wallets
- ✅ List wallets with filtering, pagination, and sorting
- ✅ Multi-currency support

### Transaction Management
- ✅ Create transactions (DEPOSIT, WITHDRAWAL, TRANSFER, PAYMENT)
- ✅ Get transaction details
- ✅ Get wallet transaction history
- ✅ List transactions with filtering, pagination, and sorting
- ✅ Update transaction status and description
- ✅ Delete transactions
- ✅ Transaction lifecycle: PENDING → SUCCESS/FAILED

### Event Streaming
- ✅ Wallet events: created, updated, deleted
- ✅ Transaction events: created, updated, succeeded, failed
- 🔄 Ready for message broker integration (Kafka, RabbitMQ)

## Database Schema

### Wallets Table
```sql
CREATE TABLE wallets (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL,
  currency VARCHAR NOT NULL DEFAULT 'USD',
  balance VARCHAR NOT NULL DEFAULT '0',
  is_active BOOLEAN NOT NULL DEFAULT true,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Transactions Table
```sql
CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  wallet_id UUID NOT NULL,
  transaction_type VARCHAR NOT NULL,
  amount VARCHAR NOT NULL,
  currency VARCHAR NOT NULL,
  status VARCHAR NOT NULL DEFAULT 'PENDING',
  reference_id VARCHAR NULL,
  description VARCHAR NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## API Endpoints

### Wallet Endpoints
- `POST /wallets` - Create wallet
- `GET /wallets/{wallet_id}` - Get wallet by ID
- `GET /wallets/user/{user_id}` - Get wallet by user ID
- `GET /wallets` - List wallets (with filtering, pagination, sorting)
- `PATCH /wallets/{wallet_id}` - Update wallet
- `DELETE /wallets/{wallet_id}` - Delete wallet

### Transaction Endpoints
- `POST /transactions` - Create transaction
- `GET /transactions/{transaction_id}` - Get transaction by ID
- `GET /wallets/{wallet_id}/transactions` - Get wallet transactions
- `GET /transactions` - List transactions (with filtering, pagination, sorting)
- `PATCH /transactions/{transaction_id}` - Update transaction
- `DELETE /transactions/{transaction_id}` - Delete transaction

## Request/Response Examples

### Create Wallet
```bash
POST /wallets
Content-Type: application/json

{
  "currency": "USD",
  "balance": "1000"
}

Response: 201 Created
{
  "ok": true,
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Create Transaction
```bash
POST /transactions
Content-Type: application/json

{
  "transaction_type": "DEPOSIT",
  "amount": "100",
  "currency": "USD",
  "description": "Payment from card",
  "reference_id": "card_123"
}

Response: 201 Created
{
  "ok": true,
  "id": "660e8400-e29b-41d4-a716-446655440001"
}
```

### Get Wallet
```bash
GET /wallets/550e8400-e29b-41d4-a716-446655440000

Response: 200 OK
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "770e8400-e29b-41d4-a716-446655440002",
  "currency": "USD",
  "balance": "1000",
  "is_active": true,
  "created_at": "2026-03-25T10:30:00",
  "updated_at": "2026-03-25T10:30:00"
}
```

## Data Models

### WalletData (DTO)
```rust
pub struct WalletData {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub currency: Option<String>,
    pub balance: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
```

### TransactionData (DTO)
```rust
pub struct TransactionData {
    pub id: Option<Uuid>,
    pub wallet_id: Option<Uuid>,
    pub transaction_type: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub reference_id: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
```

## Building & Running

### Build
```bash
cargo build --package api-wallet
```

### Run
```bash
WALLET_ENV=dev cargo run --package api-wallet
```

### Run Migration
```bash
cargo run --package features-wallet-migrations
```

### Run Tests
```bash
cargo test --package features-wallet-*
```

## Configuration

The service uses environment variables from `.env` file:

```env
WALLET_ENV=dev
WALLET_DB_URL=postgres://user:password@localhost:5432/wallet_db
WALLET_SERVER_HOST=127.0.0.1
WALLET_SERVER_PORT=8006
WALLET_JWT_SECRET=your_jwt_secret
```

## Dependencies

### Core
- `axum` - Web framework
- `tokio` - Async runtime
- `sea-orm` - ORM for database
- `sqlx` - SQL toolkit with PostgreSQL support
- `serde` - Serialization/deserialization
- `uuid` - UUID generation
- `chrono` - DateTime handling
- `tracing` - Structured logging

### Utilities
- `utoipa` - OpenAPI documentation
- `validator` - Data validation
- `async-trait` - Async trait support

## Events (Stream)

### Wallet Events
```rust
WalletEvent::Created(WalletCreatedEvent) // Wallet created
WalletEvent::Updated(WalletUpdatedEvent) // Wallet updated
WalletEvent::Deleted(WalletDeletedEvent) // Wallet deleted
```

### Transaction Events
```rust
TransactionEvent::Created(TransactionCreatedEvent)     // Transaction created
TransactionEvent::Updated(TransactionUpdatedEvent)     // Status/description updated
TransactionEvent::Succeeded(TransactionSucceededEvent) // Transaction succeeded
TransactionEvent::Failed(TransactionFailedEvent)       // Transaction failed
```

## Development Notes

### Adding New Features
1. Add database migrations in `features/wallet/migrations/src/`
2. Update entity models in `features/wallet/entities/src/`
3. Create new DTOs in `features/wallet/model/src/`
4. Add repository queries/mutations in `features/wallet/repo/src/`
5. Implement business logic in `features/wallet/service/src/`
6. Create REST endpoints in `apis/wallet/src/routes/`

### Key Design Patterns
- **Repository Pattern** - Abstracts data access
- **Service Layer** - Contains business logic
- **DTO/Model Layer** - Separates API contracts from database
- **Async/Await** - All operations are non-blocking
- **Error Handling** - Consistent `AppError` type
- **Event Sourcing** - Events published for state changes

## TODO/Future Enhancements

- [ ] Implement message broker integration (Kafka/RabbitMQ) for event publishing
- [ ] Add authentication middleware for user context
- [ ] Implement transaction rollback/reconciliation
- [ ] Add balance validation and constraints
- [ ] Implement transaction rate limiting
- [ ] Add webhook support for transaction notifications
- [ ] Payment gateway integration (Stripe, PayPal)
- [ ] Currency conversion support
- [ ] Analytics and reporting endpoints
- [ ] Load testing and performance optimization

## Testing

```bash
# Run all tests
cargo test --package features-wallet-*

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific test
cargo test --package features-wallet-service -- --exact test_name
```

## Swagger Documentation

Once the service is running, access the interactive API documentation at:
```
http://localhost:8006/swagger-ui/
```

## Contributing

Follow the existing patterns in the codebase:
1. Use the macro decorators for automatic DTO generation
2. Keep services stateless
3. Always use explicit error handling
4. Add OpenAPI documentation to endpoints
5. Follow Rust naming conventions

## License

Internal project for dn-ms microservices
