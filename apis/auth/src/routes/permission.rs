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

use features_auth_entities::permission::PermissionForCreateDto;
use features_auth_model::{
    permission::{
        PermissionData, PermissionDataFilterParams, PermissionDataResponse,
        PermissionForCreateRequest,
    },
    state::{AuthAppState, AuthCacheState},
};
use features_auth_repo::permission::{PermissionMutation, PermissionQuery};

const TAG: &str = "permissions";

#[utoipa::path(
    post,
    request_body = PermissionForCreateRequest,
    path = "/permissions",
    tag = TAG,
    responses(
        (status = 200, description = "Permission is created", body = OkUuidResponse),       
    )
)]
async fn create_permission(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(register_request): ValidJson<PermissionForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: PermissionForCreateDto = register_request.into();
    let permission_id = PermissionMutation::create(&state.conn, dto).await?;
    debug!("Created permission {:?}", permission_id);
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(permission_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/permissions/{permission_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Permission is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_permission(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(permission_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PermissionMutation::delete(&state.conn, permission_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/permissions/{permission_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Permission Data", body = PermissionDataResponse),       
    )
)]
async fn get_permission(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    Path(permission_id): Path<Uuid>,
) -> Result<ResponseJson<PermissionData>> {
    let permission = PermissionQuery::get(&state.conn, permission_id).await?;
    Ok(ResponseJson(permission))
}

#[utoipa::path(
    get,
    path = "/permissions",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Permission", body = QueryResultResponse<PermissionData>),       
    )
)]
async fn filter_permissions(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<PermissionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<PermissionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = PermissionQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/permissions", post(create_permission))
        .route("/permissions/{permission_id}", delete(delete_permission))
        .route("/permissions/{permission_id}", get(get_permission))
        .route("/permissions", get(filter_permissions))
        .with_state(app_state.clone())
}
