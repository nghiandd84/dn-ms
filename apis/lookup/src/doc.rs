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
    ),
   
    tags(
        (name = "lookup-type", description = "Lookup type management"),
        (name = "lookup-item", description = "Lookup item management"),
    )
)]
pub struct ApiDoc;

pub fn open_api() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
