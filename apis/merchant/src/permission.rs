use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// MERCHANT Permission
const MERCHANT_RESOURCE: &str = "MERCHANT_MERCHANT";

define_resource_perms! {
    CanCreateMerchant => (CREATE, MERCHANT_RESOURCE),
    CanReadMerchant => (READ, MERCHANT_RESOURCE),
    CanUpdateMerchant => (UPDATE, MERCHANT_RESOURCE),
    CanDeleteMerchant => (DELETE, MERCHANT_RESOURCE)
}

// API_KEY Permission
const API_KEY_RESOURCE: &str = "MERCHANT_API_KEY";

define_resource_perms! {
    CanCreateApiKey => (CREATE, API_KEY_RESOURCE),
    CanReadApiKey => (READ, API_KEY_RESOURCE),
    CanUpdateApiKey => (UPDATE, API_KEY_RESOURCE),
    CanDeleteApiKey => (DELETE, API_KEY_RESOURCE)
}

// WEBHOOK Permission
const WEBHOOK_RESOURCE: &str = "MERCHANT_WEBHOOK";

define_resource_perms! {
    CanCreateWebhook => (CREATE, WEBHOOK_RESOURCE),
    CanReadWebhook => (READ, WEBHOOK_RESOURCE),
    CanUpdateWebhook => (UPDATE, WEBHOOK_RESOURCE),
    CanDeleteWebhook => (DELETE, WEBHOOK_RESOURCE)
}
