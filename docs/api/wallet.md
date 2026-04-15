# Wallet Service API

Manages user wallets, balances, and transactions.

## Endpoints
- `GET /wallets` — List all wallets
- `POST /wallets` — Create a new wallet
- `GET /wallets/{id}/transactions` — Get wallet transactions
- `POST /wallets/topup` — Top up wallet
- `POST /wallets/withdraw` — Withdraw from wallet

## Integrations
- Payment Core Service
- Fee Service
- Notification Service
