# Stripe Frontend Integration

How to use the Stripe payment flow APIs from the client side using Stripe.js.

## Prerequisites

Load Stripe.js in your HTML:

```html
<script src="https://js.stripe.com/v3/"></script>
```

Initialize the Stripe client once:

```typescript
const stripe = Stripe("pk_your_publishable_key");
```

## Initiate Payment Flow

### 1. Call the backend

```typescript
const res = await fetch("/flow/initiate", {
  method: "POST",
  headers: { "Content-Type": "application/json", Authorization: `Bearer ${token}` },
  body: JSON.stringify({
    user_id: "user-uuid",
    amount: 2000,
    currency: "usd",
    idempotency_key: "order-001-user-001",
    metadata: { order_id: "order-001" }
  })
});

const { payment_id, stripe_payment_intent_id, client_secret } = await res.json();
```

**Response fields:**
- `payment_id` — internal core payment ID (for your own tracking)
- `stripe_payment_intent_id` — Stripe's PaymentIntent ID (for display/debugging)
- `client_secret` — required by Stripe.js to collect payment details and confirm

### 2. Mount the Payment Element

```html
<form id="payment-form">
  <div id="payment-element"></div>
  <button id="pay-btn" type="button">Pay</button>
  <div id="error"></div>
</form>
```

```typescript
const elements = stripe.elements({ clientSecret: client_secret });
const paymentElement = elements.create("payment");
paymentElement.mount("#payment-element");
```

### 3. Confirm payment on submit

```typescript
document.getElementById("pay-btn").addEventListener("click", async () => {
  const { error } = await stripe.confirmPayment({
    elements,
    confirmParams: {
      return_url: "https://yoursite.com/payment/complete",
    },
  });

  if (error) {
    document.getElementById("error").textContent = error.message;
  }
  // If no error, Stripe redirects to return_url
});
```

### 4. Handle the redirect on return page

After confirmation, Stripe redirects to `return_url` with query params:
- `payment_intent=pi_xxx`
- `payment_intent_client_secret=pi_xxx_secret_xxx`
- `redirect_status=succeeded`

```typescript
const params = new URLSearchParams(window.location.search);
const clientSecret = params.get("payment_intent_client_secret");

if (clientSecret) {
  const { paymentIntent } = await stripe.retrievePaymentIntent(clientSecret);

  switch (paymentIntent.status) {
    case "succeeded":
      // Show success UI
      break;
    case "processing":
      // Show "payment is processing" message
      break;
    default:
      // Show failure UI
      break;
  }
}
```

## Refund Flow

Refunds are entirely server-side. No Stripe.js interaction needed.

```typescript
const res = await fetch("/flow/refund", {
  method: "POST",
  headers: { "Content-Type": "application/json", Authorization: `Bearer ${token}` },
  body: JSON.stringify({
    payment_id: "your-payment-uuid",
    amount: 1000,                    // omit for full refund
    reason: "requested_by_customer"
  })
});

const { refund_id, stripe_refund_id, status } = await res.json();

if (status === "succeeded") {
  // Show refund success UI
} else {
  // Show error/pending UI
}
```

## Sequence Diagram

```
Client                          Backend                         Stripe
  |                                |                              |
  |-- POST /flow/initiate ->|                              |
  |                                |-- create core payment        |
  |                                |-- PaymentIntent::create ---->|
  |                                |<-- client_secret ------------|
  |<-- { client_secret } ----------|                              |
  |                                                                |
  |-- stripe.confirmPayment(client_secret) ---------------------->|
  |<-- redirect to return_url ------------------------------------|
  |                                                                |
  |                                |<-- webhook (succeeded) ------|
  |                                |-- update payment status      |
```

## Key Points

- `client_secret` is used by Stripe.js to securely confirm the payment on the client side
- `stripe_payment_intent_id` is for tracking/display — not needed by Stripe.js
- The webhook (`POST /flow/webhook`) reconciles the final payment status server-side
- Refunds don't need client-side Stripe.js — just call the API and check the response status
