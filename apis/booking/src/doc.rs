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
        crate::routes::booking_seat::create_booking_seat,
        crate::routes::booking_seat::get_booking_seat,
        crate::routes::booking_seat::filter_booking_seats,
        crate::routes::booking_seat::update_booking_seat,
        crate::routes::booking_seat::delete_booking_seat,
    ),
    tags(
        (name = "booking", description = "Booking management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
