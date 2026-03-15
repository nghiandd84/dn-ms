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
        // Add route paths here
    ),
    tags(
        (name = "stripe", description = "Stripe payment endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;