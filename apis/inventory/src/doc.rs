use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Inventory API",
        version = "0.1.0",
        description = "Complete Inventory Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::seat::create_seat,
        crate::routes::seat::get_seat,
        crate::routes::seat::filter_seats,
        crate::routes::seat::update_seat,
        crate::routes::seat::delete_seat,
        crate::routes::reservation::create_reservation,
        crate::routes::reservation::get_reservation,
        crate::routes::reservation::filter_reservations,
        crate::routes::reservation::update_reservation,
        crate::routes::reservation::delete_reservation,
    ),
    tags(
        (name = "inventory", description = "Inventory management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
