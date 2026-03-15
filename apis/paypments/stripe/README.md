# Stripe Payment API

This is a Rust API service for handling payments via Stripe.com.

## Features

- **Checkout**: Create a checkout session for clients to pay with amount, success and cancel URLs.
- **Refund**: Admin can refund payments, with check to prevent duplicate refunds.
- **View Transactions**: Admin can view all payment transactions and history.
- **Webhook**: Receive and handle events from Stripe.

## Endpoints

- `POST /checkout`: Create checkout session
  - Body: `{ "amount": 1000, "currency": "usd", "success_url": "...", "cancel_url": "..." }`
- `POST /refund`: Refund a payment
  - Body: `{ "payment_intent_id": "...", "amount": 500 }`
- `GET /payments`: List all payments
- `POST /webhook`: Handle Stripe webhooks

## Environment Variables

- `STRIPE_SECRET_KEY`: Your Stripe secret key
- `STRIPE_WEBHOOK_SECRET`: Webhook endpoint secret
- `PORT`: Port to run on (default 3001)

## Running

```bash
cargo run
```

## Dependencies

- Axum for web framework
- async-stripe (0.41) for Stripe integration