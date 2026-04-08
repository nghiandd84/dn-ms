use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::lookup_type::create_lookup_type,
        crate::routes::lookup_type::get_lookup_types,
        crate::routes::lookup_type::get_lookup_type,
        crate::routes::lookup_type::update_lookup_type,
        crate::routes::lookup_type::delete_lookup_type,
        crate::routes::lookup_item::get_lookup_items,
        crate::routes::lookup_item::get_lookup_item,
        crate::routes::lookup_item::create_lookup_item,
        crate::routes::lookup_item::update_lookup_item,
        crate::routes::lookup_item::delete_lookup_item,
        crate::routes::lookup_item_translation::get_translations,
        crate::routes::lookup_item_translation::create_translation,
        crate::routes::lookup_item_translation::update_translation,
        crate::routes::lookup_item_translation::delete_translation,
    ),
    tags(
        (name = "lookup-type", description = "Lookup type management"),
        (name = "lookup-item", description = "Lookup item management"),
    )
)]
pub struct ApiDoc;
