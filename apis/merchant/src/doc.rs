use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Merchant API",
        version = "0.1.0",
        description = "Merchant management RESTful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::merchant::create_merchant,
        crate::routes::merchant::get_merchant,
        crate::routes::merchant::filter_merchants,
        crate::routes::merchant::update_merchant,
        crate::routes::merchant::delete_merchant,
        crate::routes::merchant::get_api_keys_by_merchant,
        crate::routes::api_key::create_api_key,
        crate::routes::api_key::get_api_key,
        crate::routes::api_key::filter_api_keys,
        crate::routes::api_key::update_api_key,
        crate::routes::api_key::delete_api_key,
        crate::routes::webhook::create_webhook,
        crate::routes::webhook::get_webhook,
        crate::routes::webhook::filter_webhooks,
        crate::routes::webhook::update_webhook,
        crate::routes::webhook::delete_webhook,
    ),
    tags(
        (name = "merchant", description = "Merchant management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
