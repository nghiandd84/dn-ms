use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PayPal API",
        version = "0.1.0",
        description = "PayPal Payment Integration API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::paypal_api_log::create_api_log,
        crate::routes::paypal_api_log::get_api_log,
        crate::routes::paypal_api_log::filter_api_logs,
        crate::routes::paypal_api_log::update_api_log,
        crate::routes::paypal_api_log::delete_api_log,
        crate::routes::paypal_webhook_event::create_webhook_event,
        crate::routes::paypal_webhook_event::get_webhook_event,
        crate::routes::paypal_webhook_event::filter_webhook_events,
        crate::routes::paypal_webhook_event::update_webhook_event,
        crate::routes::paypal_webhook_event::delete_webhook_event,
        crate::routes::paypal_order::create_order,
        crate::routes::paypal_order::get_order,
        crate::routes::paypal_order::filter_orders,
        crate::routes::paypal_order::update_order,
        crate::routes::paypal_order::delete_order,
        crate::routes::paypal_refund::create_refund,
        crate::routes::paypal_refund::get_refund,
        crate::routes::paypal_refund::filter_refunds,
        crate::routes::paypal_refund::update_refund,
        crate::routes::paypal_refund::delete_refund,
        crate::routes::payment_flow::initiate_payment,
        crate::routes::payment_flow::capture_payment,
        crate::routes::payment_flow::handle_webhook,
        crate::routes::payment_flow::refund_payment,
    ),
    tags(
        (name = "paypal", description = "PayPal payment endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
