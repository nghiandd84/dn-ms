use shared_shared_app::doc::JwtSecurityAddon;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&JwtSecurityAddon),
    paths(
        crate::routes::tag_group::get_tag_groups,
        crate::routes::tag_group::get_tag_group,
        crate::routes::tag_group::create_tag_group,
        crate::routes::tag_group::update_tag_group,
        crate::routes::tag_group::delete_tag_group,
        crate::routes::tag::get_tags,
        crate::routes::tag::search_tags,
        crate::routes::tag::get_tag,
        crate::routes::tag::create_tag,
        crate::routes::tag::update_tag,
        crate::routes::tag::delete_tag,
        crate::routes::tag::get_tag_usage,
        crate::routes::tag::merge_tag,
        crate::routes::entity_tag::assign_tag,
        crate::routes::entity_tag::bulk_assign_tags,
        crate::routes::entity_tag::bulk_remove_tags,
        crate::routes::entity_tag::get_tags_for_entity,
        crate::routes::entity_tag::get_entities_for_tag,
    ),
    tags(
        (name = "tag-group", description = "Tag group management (hierarchy)"),
        (name = "tag", description = "Tag management (CRUD, search, merge)"),
        (name = "entity-tag", description = "Entity-tag association management (assign, bulk, query)"),
    )
)]
pub struct ApiDoc;
