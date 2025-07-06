use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth API",
        version = "0.1.0",
        description = "Complete Auth Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::profile::change_profile,
        crate::routes::register::register,
        crate::routes::role::create_role,
        crate::routes::role::delete_role,
        crate::routes::role::filter_roles,
        crate::routes::role::get_role,
        crate::routes::scope::create_scope,
        crate::routes::scope::delete_scope,
        crate::routes::scope::filter_scopes,
        crate::routes::scope::get_scope,
        crate::routes::scope::update_scope,
        crate::routes::user::delete_user,
        crate::routes::user::filter_users,
        crate::routes::user::get_user,
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
    modifiers(&JwtSecurityAddon)
)]
pub struct ApiDoc;
