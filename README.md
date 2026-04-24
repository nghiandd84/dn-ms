# DN Microservices

A Rust-based microservices platform built with Axum, SeaORM, and Pingora.

## Architecture

```
apps/           → Standalone applications (gateway, notification, auth-web)
apis/           → HTTP API services
features/       → Domain feature modules (entities, model, repo, service, migrations)
libs/           → Shared libraries and tools
docker/         → Docker configs and compose files
```

## Features

| Feature | Description |
|---------|-------------|
| Auth | Authentication, OAuth2, roles, permissions |
| Booking | Booking and seat management |
| Event | Event management |
| Inventory | Seat and reservation management |
| Merchant | Merchant, API keys, webhooks |
| Fee | Fee configuration |
| Profiles | User profiles and preferences |
| Translation | i18n key/value management |
| Email Template | Email templates with placeholders and translations |
| Notification | Real-time notifications (Kafka + WebSocket) |
| Bakery | Sample bakery domain (demo) |
| Lookup | Multi-tenant lookup data management |
| Payments | Payment core + Stripe integration |
| Wallet | Wallet, top-ups, transfers, withdrawals |

## Tech Stack

- **Runtime**: Rust 1.85+, Tokio
- **HTTP**: Axum, Pingora (gateway)
- **ORM**: SeaORM (PostgreSQL)
- **Cache**: Redis
- **Messaging**: Kafka (rdkafka)
- **Observability**: OpenTelemetry, Tracing
- **Auth**: JWT (jsonwebtoken), Argon2
- **API Docs**: Utoipa + Swagger UI
- **Service Discovery**: Consul

## Prerequisites

- Rust 1.85+
- Docker & Docker Compose
- PostgreSQL, Redis, Kafka (via Docker)

## Getting Started

### 1. Start infrastructure

```bash
docker-compose -f docker-compose.no_api.yml up -d
```

### 2. Run a service

```bash
cargo run --bin api-auth
```

Or start all services:

```bash
./start.sh
```

### 3. Run migrations

Migrations run automatically on service startup, or manually:

```bash
cargo run --bin migrations_auth
```

## Development

### Create a new API service

```bash
cargo new --bin --vcs none --name api-<name> apis/<name>
```

### Create a new shared library

```bash
cargo new --lib --vcs none --name shared-shared-<name> libs/shared/shared/<name>
```

### Generate a new migration

```bash
sea-orm-cli migrate init -d ./features/<name>/migrations
```

### Run tests

```bash
cargo test -p <package-name>
```

### Code coverage

```bash
cargo tarpaulin -p <package-name> --out stdout
```

## Infrastructure URLs (local)

| Service | URL |
|---------|-----|
| Redis Commander | http://localhost:8081 |
| pgAdmin | http://localhost:5050 |
