use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Fee API",
        version = "0.1.0",
        description = "Fee management RESTful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::fee::create_fee_configuration,
        crate::routes::fee::get_fee_configuration,
        crate::routes::fee::filter_fee_configurations,
        crate::routes::fee::update_fee_configuration,
        crate::routes::fee::delete_fee_configuration,
    ),
    tags(
        (name = "fee", description = "Fee management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;