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
        crate::routes::email_template::create_email_template,
        crate::routes::email_template::get_email_template,
        crate::routes::email_template::filter_email_template,
        crate::routes::email_template::update_email_template,
        crate::routes::email_template::delete_email_template,
    ),
    tags(
        (name = "Rust REST API", description = "Email Template in Rust Endpoints")
    ),
    modifiers(&JwtSecurityAddon)
)]
pub struct ApiDoc;
