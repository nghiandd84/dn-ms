use axum::{
    extract::{Path, Query},
    routing::{delete, get, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_auth_model::{
    access::AssignRoleToUserRequest,
    state::{AuthAppState, AuthCacheState},
    user::{UserData, UserDataFilterParams, UserDataResponse},
};

use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
    query_params::QueryParams,
};

use features_auth_repo::user::{UserMutation, UserQuery};
use features_auth_service::UserService;

use crate::permission::{CanDeleteUser, CanReadUser, CanUpdateUser};

const TAG: &str = "user";

#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    tag = TAG,
    summary = "Delete user",
    responses(
        (status = 200, description = "User is deleted", body = OkUuidResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn delete_user(
    _auth: Auth<CanDeleteUser>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    UserMutation::delete_user(user_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = TAG,
    summary = "Get user by ID",
    responses(
        (status = 200, description = "User Data", body = UserDataResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn get_user(
    _auth: Auth<CanReadUser>,
    Path(user_id): Path<Uuid>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<UserData>> {
    let user_dto = UserQuery::get(user_id, &query_params).await?;
    Ok(ResponseJson(user_dto))
}

#[utoipa::path(
    get,
    path = "/users",
    tag = TAG,
    summary = "Filter users",
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered user data", body = QueryResultResponse<UserData>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_users(
    _auth: Auth<CanReadUser>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<UserDataFilterParams>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<UserData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = UserQuery::search(&pagination, &order, &all_filters, &query_params).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    request_body = AssignRoleToUserRequest,
    path = "/users/{user_id}/assign-roles",
    tag = TAG,
    summary = "Assign roles to user",
    responses(
        (status = 200, description = "Roles were assigned", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn assign_roles(
    _auth: Auth<CanUpdateUser>,
    Path(user_id): Path<Uuid>,
    ValidJson(request): ValidJson<AssignRoleToUserRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let assign = UserService::assign_roles(user_id, request.role_ids, request.key).await?;
    Ok(ResponseJson(OkUuid {
        ok: assign,
        id: None,
    }))
}

#[utoipa::path(
    post,
    request_body = AssignRoleToUserRequest,
    path = "/users/{user_id}/unassign-roles",
    tag = TAG,
    summary = "Unassign roles from user",
    responses(
        (status = 200, description = "Roles were unassigned", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn unassign_roles(
    _auth: Auth<CanUpdateUser>,
    Path(user_id): Path<Uuid>,
    ValidJson(request): ValidJson<AssignRoleToUserRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let unassign = UserService::unassign_roles(user_id, request.role_ids).await?;
    Ok(ResponseJson(OkUuid {
        ok: unassign,
        id: None,
    }))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/users/{user_id}", delete(delete_user))
        .route("/users/{user_id}", get(get_user))
        .route("/users", get(filter_users))
        .route("/users/{user_id}/assign-roles", post(assign_roles))
        .route("/users/{user_id}/unassign-roles", post(unassign_roles))
        .with_state(app_state.clone())
}
