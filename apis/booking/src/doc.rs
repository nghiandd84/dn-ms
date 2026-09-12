use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Booking API",
        version = "0.1.0",
        description = "Complete Booking Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::booking::create_booking,
        crate::routes::booking::get_booking,
        crate::routes::booking::filter_bookings,
        crate::routes::booking::update_booking,
        crate::routes::booking::delete_booking,
        crate::routes::booking::get_booking_history,
        crate::routes::booking_item::create_booking_item,
        crate::routes::booking_item::get_booking_item,
        crate::routes::booking_item::filter_booking_items,
        crate::routes::booking_item::update_booking_item,
        crate::routes::booking_item::delete_booking_item,
    ),
    tags(
        (name = "booking", description = "Booking management endpoints"),
        (name = "booking_item", description = "Booking line item endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
