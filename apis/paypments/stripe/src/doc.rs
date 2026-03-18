use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Stripe API",
        version = "0.1.0",
        description = "Stripe Payment Integration API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::stripe_api_log::create_api_log,
        crate::routes::stripe_api_log::get_api_log,
        crate::routes::stripe_api_log::filter_api_logs,
        crate::routes::stripe_api_log::update_api_log,
        crate::routes::stripe_api_log::delete_api_log,
        crate::routes::stripe_webhook_event::create_webhook_event,
        crate::routes::stripe_webhook_event::get_webhook_event,
        crate::routes::stripe_webhook_event::filter_webhook_events,
        crate::routes::stripe_webhook_event::update_webhook_event,
        crate::routes::stripe_webhook_event::delete_webhook_event,
        crate::routes::stripe_payment_intent::create_payment_intent,
        crate::routes::stripe_payment_intent::get_payment_intent,
        crate::routes::stripe_payment_intent::filter_payment_intents,
        crate::routes::stripe_payment_intent::update_payment_intent,
        crate::routes::stripe_payment_intent::delete_payment_intent,
        crate::routes::stripe_refund::create_refund,
        crate::routes::stripe_refund::get_refund,
        crate::routes::stripe_refund::filter_refunds,
        crate::routes::stripe_refund::update_refund,
        crate::routes::stripe_refund::delete_refund
    ),
    tags(
        (name = "stripe", description = "Stripe payment endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;