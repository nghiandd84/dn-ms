use utoipa::OpenApi;

use shared_shared_app::doc::JwtSecurityAddon;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Translation API",
        version = "0.1.0",
        description = "Complete Translation Management Restful API"
    ),
    paths(
        shared_shared_app::health::health_checker_handler,
        crate::routes::project::create_project,
        crate::routes::project::get_project,
        crate::routes::project::filter_projects,
        crate::routes::project::update_project,
        crate::routes::project::delete_project,
        crate::routes::translation_key::create_translation_key,
        crate::routes::translation_key::get_translation_key,
        crate::routes::translation_key::get_translation_keys_by_project,
        crate::routes::translation_key::filter_translation_keys,
        crate::routes::translation_key::update_translation_key,
        crate::routes::translation_key::delete_translation_key,
        crate::routes::tag::create_tag,
        crate::routes::tag::get_tag,
        crate::routes::tag::filter_tags,
        crate::routes::tag::update_tag,
        crate::routes::tag::delete_tag,
        crate::routes::translation_version::create_translation_version,
        crate::routes::translation_version::get_translation_version,
        crate::routes::translation_version::get_latest_translation_version,
        crate::routes::translation_version::filter_translation_versions,
        crate::routes::translation_version::update_translation_version,
        crate::routes::translation_version::delete_translation_version,
    ),
    tags(
        (name = "project", description = "Project management endpoints"),
        (name = "translation_key", description = "Translation key management endpoints"),
        (name = "tag", description = "Tag management endpoints"),
        (name = "translation_version", description = "Translation version management endpoints"),
    ),
    modifiers(&JwtSecurityAddon),
)]
pub struct ApiDoc;
