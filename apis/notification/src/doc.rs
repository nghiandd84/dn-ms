use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Email Template API",
        version = "0.1.0",
        description = "Email Template Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
    ),
    tags(
        (name = "Rust REST API", description = "Email Template in Rust Endpoints")
    ),
    modifiers(&JwtSecurityAddon)
)]
pub struct ApiDoc;
