use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_url_shortener_model::{
    shortened_url::{
        CreateShortenedUrlRequest, ShortenedUrlData, ShortenedUrlDataFilterParams,
        UpdateShortenedUrlRequest,
    },
    state::{UrlShortenerAppState, UrlShortenerCacheState},
};
use features_url_shortener_service::shortened_url::ShortenedUrlService;

use crate::permission::{CanCreateUrl, CanDeleteUrl, CanUpdateUrl};

const TAG: &str = "shortened-url";

#[utoipa::path(
    post,
    path = "/urls",
    tag = TAG,
    request_body = CreateShortenedUrlRequest,
    responses(
        (status = 201, description = "URL created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_url(
    auth: Auth<CanCreateUrl>,
    ValidJson(mut req): ValidJson<CreateShortenedUrlRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let user_id = auth.user_id;
    debug!("Creating shortened URL for user: {}", user_id);
    req.user_id = Some(user_id);
    let id = ShortenedUrlService::create_short_url(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    get,
    path = "/urls",
    tag = TAG,
    params(Pagination, Order),
    responses(
        (status = 200, description = "List of user's URLs", body = QueryResultResponse<ShortenedUrlData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_urls(
    auth: Auth<CanCreateUrl>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<ShortenedUrlDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ShortenedUrlData>>> {
    let user_id = auth.user_id;
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        ShortenedUrlService::list_user_urls(&user_id, &pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/urls/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "URL details", body = ShortenedUrlData),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_url(
    _auth: Auth<CanCreateUrl>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<ShortenedUrlData>> {
    let result =
        features_url_shortener_repo::ShortenedUrlQuery::get_by_id(id, &Default::default()).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/urls/{id}",
    tag = TAG,
    request_body = UpdateShortenedUrlRequest,
    responses(
        (status = 200, description = "URL updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_url(
    auth: Auth<CanUpdateUrl>,
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<UpdateShortenedUrlRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let user_id = auth.user_id;
    ShortenedUrlService::update_short_url(id, user_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/urls/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "URL deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_url(
    auth: Auth<CanDeleteUrl>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    let user_id = auth.user_id;
    ShortenedUrlService::delete_short_url(id, user_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<UrlShortenerAppState, UrlShortenerCacheState>) -> Router {
    Router::new()
        .route("/urls", post(create_url))
        .route("/urls", get(get_urls))
        .route("/urls/{id}", get(get_url))
        .route("/urls/{id}", patch(update_url))
        .route("/urls/{id}", delete(delete_url))
        .with_state(app_state.clone())
}
