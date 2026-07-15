use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::shortened_url::create_url,
        crate::routes::shortened_url::get_urls,
        crate::routes::shortened_url::get_url,
        crate::routes::shortened_url::update_url,
        crate::routes::shortened_url::delete_url,
        crate::routes::redirect::redirect_to_url,
        crate::routes::url_click::get_url_clicks,
        crate::routes::api_key::create_api_key,
        crate::routes::api_key::get_api_keys,
        crate::routes::api_key::delete_api_key,
    ),
    tags(
        (name = "shortened-url", description = "URL shortener management"),
        (name = "redirect", description = "Public URL redirection"),
        (name = "url-click", description = "URL analytics"),
        (name = "api-key", description = "API key management"),
    )
)]
pub struct ApiDoc;
