# Stripe Service

Integrates with Stripe for payment processing, webhooks, and reconciliation.
- Handles Stripe payment flows and webhooks
- Maps Stripe events to internal payment and wallet updates
- Used by payment-core and wallet services
- Typical endpoints: `/stripe/webhook`, `/stripe/pay`
