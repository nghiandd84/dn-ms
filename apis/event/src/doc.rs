use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Event API",
        version = "0.1.0",
        description = "Complete Event Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::event::create_event,
        crate::routes::event::get_event,
        crate::routes::event::filter_events,
        crate::routes::event::update_event,
        crate::routes::event::delete_event,
        crate::routes::public_event::search_public_events,
        crate::routes::public_event::get_public_event,
    ),
    tags(
        (name = "event", description = "Event management endpoints"),
        (name = "public-event", description = "Public event endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
