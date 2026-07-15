use axum::{
    extract::Path,
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use tracing::{debug, instrument, Level};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::PublicAccess;

use features_url_shortener_model::state::{UrlShortenerAppState, UrlShortenerCacheState};
use features_url_shortener_service::shortened_url::ShortenedUrlService;

use crate::error_page::render_error_page;

#[utoipa::path(
    get,
    path = "/r/{code}",
    tag = "redirect",
    responses(
        (status = 302, description = "Redirect to original URL"),
        (status = 404, description = "URL not found or expired"),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn redirect_to_url(
    _public: PublicAccess,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Response {
    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let referrer = headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    debug!("Redirect request for code: {}", code);

    match ShortenedUrlService::redirect(&code, ip_address, user_agent, referrer).await {
        Ok(original_url) => Redirect::temporary(&original_url).into_response(),
        Err(e) => {
            let message = match &e {
                shared_shared_data_error::app::AppError::EntityNotFound { entity } => {
                    entity.clone()
                }
                _ => "This link is not available".to_string(),
            };
            Html(render_error_page(&message)).into_response()
        }
    }
}

pub fn routes(app_state: &AppState<UrlShortenerAppState, UrlShortenerCacheState>) -> Router {
    Router::new()
        .route("/r/{code}", get(redirect_to_url))
        .with_state(app_state.clone())
}
