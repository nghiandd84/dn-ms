use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// PAYMENT_INTENT Permission
const PAYMENT_INTENT_RESOURCE: &str = "STRIPE_PAYMENT_INTENT";

define_resource_perms! {
    CanCreatePaymentIntent => (CREATE, PAYMENT_INTENT_RESOURCE),
    CanReadPaymentIntent => (READ, PAYMENT_INTENT_RESOURCE),
    CanUpdatePaymentIntent => (UPDATE, PAYMENT_INTENT_RESOURCE),
    CanDeletePaymentIntent => (DELETE, PAYMENT_INTENT_RESOURCE)
}

// WEBHOOK_EVENT Permission
const WEBHOOK_EVENT_RESOURCE: &str = "STRIPE_WEBHOOK_EVENT";

define_resource_perms! {
    CanCreateWebhookEvent => (CREATE, WEBHOOK_EVENT_RESOURCE),
    CanReadWebhookEvent => (READ, WEBHOOK_EVENT_RESOURCE),
    CanUpdateWebhookEvent => (UPDATE, WEBHOOK_EVENT_RESOURCE),
    CanDeleteWebhookEvent => (DELETE, WEBHOOK_EVENT_RESOURCE)
}

// REFUND Permission
const REFUND_RESOURCE: &str = "STRIPE_REFUND";

define_resource_perms! {
    CanCreateRefund => (CREATE, REFUND_RESOURCE),
    CanReadRefund => (READ, REFUND_RESOURCE),
    CanUpdateRefund => (UPDATE, REFUND_RESOURCE),
    CanDeleteRefund => (DELETE, REFUND_RESOURCE)
}

// API_LOG Permission
const API_LOG_RESOURCE: &str = "STRIPE_API_LOG";

define_resource_perms! {
    CanCreateApiLog => (CREATE, API_LOG_RESOURCE),
    CanReadApiLog => (READ, API_LOG_RESOURCE),
    CanUpdateApiLog => (UPDATE, API_LOG_RESOURCE),
    CanDeleteApiLog => (DELETE, API_LOG_RESOURCE)
}
