# Microservice Purpose Summary

This document summarizes the purpose and main responsibilities of each microservice in the dn-ms monorepo.

---


## Auth Service
Handles authentication, authorization, and user identity management.
- Issues and validates JWT tokens
- Manages user credentials (registration, password reset, MFA)
- Enforces permissions and roles
- Integrates with profile and notification services for onboarding and alerts
- Typical endpoints: `/login`, `/register`, `/token/refresh`, `/me`, `/permissions`


## Booking Service
Manages booking operations, schedules, and reservations.
- Handles creation, modification, and cancellation of bookings
- Supports querying available slots and booking history
- Integrates with wallet (for payments) and notification (for reminders)
- Typical endpoints: `/bookings`, `/bookings/{id}`, `/bookings/history`, `/slots`


## Wallet Service
Manages user wallets, balances, and transactions.
- Wallet creation, top-up, withdrawal, and transfer
- Transaction history and balance queries
- Integrates with payment-core, fee, and notification services
- Typical endpoints: `/wallets`, `/wallets/{id}/transactions`, `/wallets/topup`, `/wallets/withdraw`


## Inventory Service
Tracks inventory items, stock levels, and product availability.
- CRUD for inventory items and categories
- Stock level updates and low-stock alerts
- Integrates with booking and merchant services
- Typical endpoints: `/inventory`, `/inventory/{id}`, `/inventory/stock`


## Merchant Service
Handles merchant onboarding, profile management, and merchant-specific operations.
- Merchant registration, verification, and profile updates
- Links merchants to products, inventory, and bookings
- Integrates with fee and payment services
- Typical endpoints: `/merchants`, `/merchants/{id}`, `/merchants/onboard`


## Fee Service
Calculates and manages service fees, commissions, and related financial rules.
- Fee rule definition and lookup
- Applies fees to wallet and payment transactions
- Integrates with wallet, payment-core, and merchant services
- Typical endpoints: `/fees`, `/fees/calculate`, `/fees/rules`


## Notification Service
Sends notifications (email, SMS, push) to users and merchants.
- Manages notification templates and delivery status
- Supports event-driven and scheduled notifications
- Integrates with all user-facing services (auth, booking, wallet, etc.)
- Typical endpoints: `/notifications`, `/notifications/send`, `/templates`


## Profile Service
Manages user profiles, preferences, and personal information.
- CRUD for user profiles and preferences
- Profile enrichment and verification
- Integrates with auth and notification services
- Typical endpoints: `/profiles`, `/profiles/{id}`, `/profiles/preferences`


## Translation Service
Provides translation and localization services for multi-language support.
- Manages translation keys and values
- Supports dynamic and static translations
- Integrates with all user-facing APIs for i18n
- Typical endpoints: `/translate`, `/languages`, `/translations/{key}`


## Lookup Service
Offers lookup tables and reference data (e.g., country codes, status enums) for other services.
- CRUD for lookup tables and values
- Centralizes enums and reference data for consistency
- Used by all services for validation and display
- Typical endpoints: `/lookup/{type}`, `/lookup/{type}/{id}`


## Bakery Service
Domain-specific service for bakery operations (e.g., product catalog, orders, recipes).
- Manages bakery products, recipes, and orders
- Integrates with inventory and booking services
- Typical endpoints: `/bakery/products`, `/bakery/orders`, `/bakery/recipes`


## Email Template Service
Manages email templates for transactional and marketing emails.
- CRUD for email templates
- Supports versioning and preview
- Used by notification service for email delivery
- Typical endpoints: `/email-templates`, `/email-templates/{id}`


## Event Service
Handles event publishing, subscription, and processing for inter-service communication.

Core payment processing logic, transaction orchestration, and integration with payment providers.
- Handles payment initiation, status tracking, and reconciliation
## Stripe Service
Integrates with Stripe for payment processing, webhooks, and reconciliation.
- Handles Stripe payment flows and webhooks
- Maps Stripe events to internal payment and wallet updates
- Used by payment-core and wallet services
- Typical endpoints: `/stripe/webhook`, `/stripe/pay`

---

Each service is designed to be self-contained, with clear boundaries and responsibilities. For more details, see the corresponding API and feature documentation for each service.
