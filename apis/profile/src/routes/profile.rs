use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_profiles_model::{
    state::{ProfileAppState, ProfileCacheState},
    ProfileData, ProfileDataFilterParams, ProfileForCreateRequest, ProfileForUpdateRequest,
};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use crate::permission::{CanCreateProfile, CanDeleteProfile, CanReadProfile, CanUpdateProfile};
use features_profiles_service::ProfileService;

const TAG: &str = "profile";

#[utoipa::path(
    post,
    path = "/profiles",
    tag = TAG,
    request_body = ProfileForCreateRequest,
    responses(
        (status = 201, description = "Profile created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_profile(_auth: Auth<CanCreateProfile>, Json(req): Json<ProfileForCreateRequest>) -> Result<ResponseJson<OkUuid>> {
    let profile_id = ProfileService::create_profile(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(profile_id),
    }))
}

#[utoipa::path(
    get,
    path = "/profiles/{profile_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Profile retrieved successfully", body = ProfileData),
    )
)]
async fn get_profile(_auth: Auth<CanReadProfile>, Path(profile_id): Path<Uuid>) -> Result<ResponseJson<ProfileData>> {
    let profile = ProfileService::get_profile_by_id(profile_id).await?;
    Ok(ResponseJson(profile))
}

#[utoipa::path(
    get,
    path = "/profiles/user/{user_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Profile retrieved by user ID", body = ProfileData),
    )
)]
async fn get_profile_by_user_id(_auth: Auth<CanReadProfile>, Path(user_id): Path<Uuid>) -> Result<ResponseJson<ProfileData>> {
    let profile = ProfileService::get_profile_by_user_id(user_id).await?;
    Ok(ResponseJson(profile))
}

#[utoipa::path(
    get,
    path = "/profiles",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered profiles", body = QueryResultResponse<ProfileData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_profiles(
    _auth: Auth<CanReadProfile>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<ProfileDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ProfileData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = ProfileService::get_profiles(pagination, order, filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/profiles/{profile_id}",
    tag = TAG,
    request_body = ProfileForUpdateRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = OkUuidResponse),
    )
)]
async fn update_profile(
    _auth: Auth<CanUpdateProfile>,
    Path(profile_id): Path<Uuid>,
    Json(req): Json<ProfileForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ProfileService::update_profile(profile_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(profile_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/profiles/{profile_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Profile deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_profile(_auth: Auth<CanDeleteProfile>, Path(profile_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    ProfileService::delete_profile(profile_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(profile_id),
    }))
}

pub fn routes(app_state: &AppState<ProfileAppState, ProfileCacheState>) -> Router {
    Router::new()
        .route("/profiles", post(create_profile))
        .route("/profiles", get(filter_profiles))
        .route("/profiles/{profile_id}", get(get_profile))
        .route("/profiles/user/{user_id}", get(get_profile_by_user_id))
        .route("/profiles/{profile_id}", patch(update_profile))
        .route("/profiles/{profile_id}", delete(delete_profile))
        .with_state(app_state.clone())
}
