use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// ORDER Permission
const ORDER_RESOURCE: &str = "PAYMENT_PAYPAL:ORDER";

define_resource_perms! {
    CanCreateOrder => (CREATE, ORDER_RESOURCE),
    CanReadOrder => (READ, ORDER_RESOURCE),
    CanUpdateOrder => (UPDATE, ORDER_RESOURCE),
    CanDeleteOrder => (DELETE, ORDER_RESOURCE)
}

// WEBHOOK_EVENT Permission
const WEBHOOK_EVENT_RESOURCE: &str = "PAYMENT_PAYPAL:WEBHOOK_EVENT";

define_resource_perms! {
    CanCreateWebhookEvent => (CREATE, WEBHOOK_EVENT_RESOURCE),
    CanReadWebhookEvent => (READ, WEBHOOK_EVENT_RESOURCE),
    CanUpdateWebhookEvent => (UPDATE, WEBHOOK_EVENT_RESOURCE),
    CanDeleteWebhookEvent => (DELETE, WEBHOOK_EVENT_RESOURCE)
}

// REFUND Permission
const REFUND_RESOURCE: &str = "PAYMENT_PAYPAL:REFUND";

define_resource_perms! {
    CanCreateRefund => (CREATE, REFUND_RESOURCE),
    CanReadRefund => (READ, REFUND_RESOURCE),
    CanUpdateRefund => (UPDATE, REFUND_RESOURCE),
    CanDeleteRefund => (DELETE, REFUND_RESOURCE)
}

// API_LOG Permission
const API_LOG_RESOURCE: &str = "PAYMENT_PAYPAL:API_LOG";

define_resource_perms! {
    CanCreateApiLog => (CREATE, API_LOG_RESOURCE),
    CanReadApiLog => (READ, API_LOG_RESOURCE),
    CanUpdateApiLog => (UPDATE, API_LOG_RESOURCE),
    CanDeleteApiLog => (DELETE, API_LOG_RESOURCE)
}
