use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_profiles_model::{
    state::{ProfileAppState, ProfileCacheState},
    SocialLinkData, SocialLinkDataFilterParams, SocialLinkForCreateRequest,
    SocialLinkForUpdateRequest,
};
use features_profiles_service::SocialLinkService;

const TAG: &str = "social-link";

#[utoipa::path(
    post,
    path = "/social-links",
    tag = TAG,
    request_body = SocialLinkForCreateRequest,
    responses(
        (status = 201, description = "Social link created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_social_link(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Json(req): Json<SocialLinkForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let link_id = SocialLinkService::create_social_link(&state.conn, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(link_id),
    }))
}

#[utoipa::path(
    get,
    path = "/social-links/{link_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Social link retrieved successfully", body = SocialLinkData),
    )
)]
async fn get_social_link(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(link_id): Path<Uuid>,
) -> Result<ResponseJson<SocialLinkData>> {
    let link = SocialLinkService::get_social_link_by_id(&state.conn, link_id).await?;
    Ok(ResponseJson(link))
}

#[utoipa::path(
    get,
    path = "/social-links/profile/{profile_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Social links retrieved by profile ID", body = Vec<SocialLinkData>),
    )
)]
async fn get_social_links_by_profile_id(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(profile_id): Path<Uuid>,
) -> Result<ResponseJson<Vec<SocialLinkData>>> {
    let links = SocialLinkService::get_social_links_by_profile_id(&state.conn, profile_id).await?;
    Ok(ResponseJson(links))
}

#[utoipa::path(
    get,
    path = "/social-links",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered social links", body = QueryResultResponse<SocialLinkData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_social_links(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<SocialLinkDataFilterParams>,
) -> Result<ResponseJson<QueryResult<SocialLinkData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();

    let result =
        SocialLinkService::get_social_links(&state.conn, pagination, order, filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/social-links/{link_id}",
    tag = TAG,
    request_body = SocialLinkForUpdateRequest,
    responses(
        (status = 200, description = "Social link updated successfully", body = OkUuidResponse),
    )
)]
async fn update_social_link(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(link_id): Path<Uuid>,
    Json(req): Json<SocialLinkForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    SocialLinkService::update_social_link(&state.conn, link_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(link_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/social-links/{link_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Social link deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_social_link(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(link_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    SocialLinkService::delete_social_link(&state.conn, link_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(link_id),
    }))
}

pub fn routes(app_state: &AppState<ProfileAppState, ProfileCacheState>) -> Router {
    Router::new()
        .route("/social-links", post(create_social_link))
        .route("/social-links", get(filter_social_links))
        .route("/social-links/{link_id}", get(get_social_link))
        .route(
            "/social-links/profile/{profile_id}",
            get(get_social_links_by_profile_id),
        )
        .route("/social-links/{link_id}", patch(update_social_link))
        .route("/social-links/{link_id}", delete(delete_social_link))
        .with_state(app_state.clone())
}
