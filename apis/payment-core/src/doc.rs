use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Payment Core API",
        version = "0.1.0",
        description = "Complete Payment Core Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::payment::create_payment,
        crate::routes::payment::get_payment,
        crate::routes::payment::filter_payments,
        crate::routes::payment::update_payment,
        crate::routes::payment::delete_payment,
        crate::routes::payment_attempt::create_payment_attempt,
        crate::routes::payment_attempt::get_payment_attempt,
        crate::routes::payment_attempt::filter_payment_attempts,
        crate::routes::payment_attempt::update_payment_attempt,
        crate::routes::payment_attempt::delete_payment_attempt,
        crate::routes::payment_method::create_payment_method,
        crate::routes::payment_method::get_payment_method,
        crate::routes::payment_method::filter_payment_methods,
        crate::routes::payment_method::update_payment_method,
        crate::routes::payment_method::delete_payment_method,
        crate::routes::payment_method_limit::create_payment_method_limit,
        crate::routes::payment_method_limit::get_payment_method_limit,
        crate::routes::payment_method_limit::filter_payment_method_limits,
        crate::routes::payment_method_limit::update_payment_method_limit,
        crate::routes::payment_method_limit::delete_payment_method_limit,
    ),
    tags(
        (name = "payment-core", description = "Payment Core management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
