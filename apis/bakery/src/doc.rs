use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bakery API",
        version = "0.1.0",
        description = "Bakery Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::baker::create,
        crate::routes::baker::delete_by_id,
        crate::routes::baker::filter,
        crate::routes::baker::get_by_id,
        crate::routes::bakery::create,
        crate::routes::bakery::delete_by_id,
        crate::routes::bakery::filter,
        crate::routes::bakery::get_by_id,
        crate::routes::cake_bakers::create,
        crate::routes::cake_bakers::delete_by_id,
        crate::routes::cake::create,
        crate::routes::cake::delete_by_id,
        crate::routes::cake::filter,
        crate::routes::cake::get_by_id,
        crate::routes::customer::create,
        crate::routes::customer::delete_by_id,
        crate::routes::customer::filter,
        crate::routes::customer::get_by_id,
        crate::routes::order::create,
        crate::routes::order::delete_by_id,
        crate::routes::order::filter,
        crate::routes::order::get_by_id,
        crate::routes::lineitem::create,
        crate::routes::lineitem::delete_by_id,
        crate::routes::lineitem::filter,
        crate::routes::lineitem::get_by_id,
    ),

    tags(
        (name = "Rest", description = "Authentication in Rust Endpoints")
    ),
    modifiers(&JwtSecurityAddon)
)]
pub struct ApiDoc;
