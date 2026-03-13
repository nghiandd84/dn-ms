use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_profiles_model::{
    UserPreferenceData, UserPreferenceDataFilterParams, UserPreferenceForCreateRequest,
    UserPreferenceForUpdateRequest,
};

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

use features_profiles_model::state::{ProfileAppState, ProfileCacheState};
use features_profiles_service::UserPreferenceService;

const TAG: &str = "user-preference";

#[utoipa::path(
    post,
    path = "/user-preferences",
    tag = TAG,
    request_body = UserPreferenceForCreateRequest,
    responses(
        (status = 201, description = "User preference created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_user_preference(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Json(req): Json<UserPreferenceForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let preference_id = UserPreferenceService::create_user_preference(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(preference_id),
    }))
}

#[utoipa::path(
    get,
    path = "/user-preferences/{preference_id}",
    tag = TAG,
    responses(
        (status = 200, description = "User preference retrieved successfully", body = UserPreferenceData),
    )
)]
async fn get_user_preference(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(preference_id): Path<Uuid>,
) -> Result<ResponseJson<UserPreferenceData>> {
    let preference =
        UserPreferenceService::get_user_preference_by_id(&state.conn, preference_id).await?;
    Ok(ResponseJson(preference))
}

#[utoipa::path(
    get,
    path = "/user-preferences/profile/{profile_id}",
    tag = TAG,
    responses(
        (status = 200, description = "User preference retrieved by profile ID", body = UserPreferenceData),
    )
)]
async fn get_user_preference_by_profile_id(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(profile_id): Path<Uuid>,
) -> Result<ResponseJson<UserPreferenceData>> {
    let preference =
        UserPreferenceService::get_user_preference_by_profile_id(&state.conn, profile_id).await?;
    Ok(ResponseJson(preference))
}

#[utoipa::path(
    get,
    path = "/user-preferences",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered user preferences", body = QueryResultResponse<UserPreferenceData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_user_preferences(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<UserPreferenceDataFilterParams>,
) -> Result<ResponseJson<QueryResult<UserPreferenceData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();

    let result =
        UserPreferenceService::get_user_preferences(&state.conn, pagination, order, filters)
            .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/user-preferences/{preference_id}",
    tag = TAG,
    request_body = UserPreferenceForUpdateRequest,
    responses(
        (status = 200, description = "User preference updated successfully", body = OkUuidResponse),
    )
)]
async fn update_user_preference(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(preference_id): Path<Uuid>,
    Json(req): Json<UserPreferenceForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    UserPreferenceService::update_user_preference(preference_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(preference_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/user-preferences/{preference_id}",
    tag = TAG,
    responses(
        (status = 200, description = "User preference deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_user_preference(
    state: State<AppState<ProfileAppState, ProfileCacheState>>,
    Path(preference_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    UserPreferenceService::delete_user_preference(preference_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(preference_id),
    }))
}

pub fn routes(app_state: &AppState<ProfileAppState, ProfileCacheState>) -> Router {
    Router::new()
        .route("/user-preferences", post(create_user_preference))
        .route("/user-preferences", get(filter_user_preferences))
        .route(
            "/user-preferences/{preference_id}",
            get(get_user_preference),
        )
        .route(
            "/user-preferences/profile/{profile_id}",
            get(get_user_preference_by_profile_id),
        )
        .route(
            "/user-preferences/{preference_id}",
            patch(update_user_preference),
        )
        .route(
            "/user-preferences/{preference_id}",
            delete(delete_user_preference),
        )
        .with_state(app_state.clone())
}
