use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Router,
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_entities::role::RoleForCreateDto;
use features_auth_model::{
    role::{
        AssignPermissionToRoleRequest, RoleData, RoleDataFilterParams, RoleDataResponse,
        RoleForCreateRequest,
    },
    state::{AuthAppState, AuthCacheState},
};
use features_auth_repo::role::{RoleMutation, RoleQuery};
use features_auth_service::RoleService;

const TAG: &str = "role";

#[utoipa::path(
    post,
    request_body = RoleForCreateRequest,
    path = "/roles",
    tag = TAG,
    responses(
        (status = 200, description = "Role is created", body = OkUuidResponse),       
    )
)]
async fn create_role(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(register_request): ValidJson<RoleForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: RoleForCreateDto = register_request.into();
    let role_id = RoleMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/roles/{role_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Role is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_role(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(role_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    RoleMutation::delete(&state.conn, role_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/roles/{role_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Role Data", body = RoleDataResponse),       
    )
)]
async fn get_role(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(role_id): Path<Uuid>,
) -> Result<ResponseJson<RoleData>> {
    let role = RoleQuery::get(&state.conn, role_id).await?;
    Ok(ResponseJson(role))
}

#[utoipa::path(
    get,
    path = "/roles",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Role", body = QueryResultResponse<RoleData>),       
    )
)]
async fn filter_roles(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<RoleDataFilterParams>,
) -> Result<ResponseJson<QueryResult<RoleData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = RoleQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    request_body = AssignPermissionToRoleRequest,
    path = "/roles/{role_id}/assign-permissions",
    tag = TAG,
    responses(
        (status = 200, description = "Permission was assigned", body = OkUuidResponse),       
    )
)]
async fn assign_permissions(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(role_id): Path<Uuid>,
    ValidJson(request): ValidJson<AssignPermissionToRoleRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let assign =
        RoleService::assign_permissions(&state.conn, role_id, request.permission_ids).await?;
    Ok(ResponseJson(OkUuid {
        ok: assign,
        id: None,
    }))
}

#[utoipa::path(
    post,
    request_body = AssignPermissionToRoleRequest,
    path = "/roles/{role_id}/unassign-permissions",
    tag = TAG,
    responses(
        (status = 200, description = "Permission was assigned", body = OkUuidResponse),       
    )
)]
async fn unassign_permissions(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(role_id): Path<Uuid>,
    ValidJson(request): ValidJson<AssignPermissionToRoleRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let assign =
        RoleService::unassign_permissions(&state.conn, role_id, request.permission_ids).await?;
    Ok(ResponseJson(OkUuid {
        ok: assign,
        id: None,
    }))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/roles", post(create_role))
        .route("/roles/{role_id}", delete(delete_role))
        .route(
            "/roles/{role_id}/assign-permissions",
            post(assign_permissions),
        )
        .route(
            "/roles/{role_id}/unassign-permissions",
            post(unassign_permissions),
        )
        .route("/roles/{role_id}", get(get_role))
        .route("/roles", get(filter_roles))
        .with_state(app_state.clone())
}
