use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_model::{
    field_permission::{
        FieldPermissionData, FieldPermissionDataFilterParams, FieldPermissionDataResponse,
        FieldPermissionForCreateRequest, FieldPermissionForUpdateRequest,
    },
    state::{AuthAppState, AuthCacheState},
};
use features_auth_service::FieldPermissionService;

use crate::permission as perm;

const TAG: &str = "field-permissions";

#[utoipa::path(
    post,
    request_body = FieldPermissionForCreateRequest,
    path = "/field-permissions",
    tag = TAG,
    summary = "Create field permission",
    description = "Create a field-level permission entry for a role on a resource",
    responses(
        (status = 200, description = "Field permission created", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn create_field_permission(
    _auth: Auth<perm::CanCreateFieldPermission>,
    ValidJson(request): ValidJson<FieldPermissionForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = FieldPermissionService::create(request).await?;
    debug!("Created field permission {:?}", id);
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    patch,
    request_body = FieldPermissionForUpdateRequest,
    params(
        ("id" = String, Path, description = "Field Permission Id"),
    ),
    path = "/field-permissions/{id}",
    tag = TAG,
    summary = "Update field permission",
    description = "Update the allowed fields for a field-level permission entry",
    responses(
        (status = 200, description = "Field permission updated", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Field permission not found", body = ErrorResponse),
    )
)]
async fn update_field_permission(
    _auth: Auth<perm::CanUpdateFieldPermission>,
    Path(id): Path<Uuid>,
    ValidJson(request): ValidJson<FieldPermissionForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    FieldPermissionService::update(id, request).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/field-permissions/{id}",
    tag = TAG,
    summary = "Delete field permission",
    responses(
        (status = 200, description = "Field permission deleted", body = OkUuidResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Field permission not found", body = ErrorResponse),
    )
)]
async fn delete_field_permission(
    _auth: Auth<perm::CanDeleteFieldPermission>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    FieldPermissionService::delete(id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/field-permissions/{id}",
    tag = TAG,
    summary = "Get field permission by ID",
    responses(
        (status = 200, description = "Field permission data", body = FieldPermissionDataResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Field permission not found", body = ErrorResponse),
    )
)]
async fn get_field_permission(
    _auth: Auth<perm::CanReadFieldPermission>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<FieldPermissionData>> {
    let data = FieldPermissionService::get(id).await?;
    Ok(ResponseJson(data))
}

#[utoipa::path(
    get,
    path = "/field-permissions",
    tag = TAG,
    summary = "Search field permissions",
    params(Order, Pagination),
    responses(
        (status = 200, description = "Filtered field permissions", body = QueryResultResponse<FieldPermissionData>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn search_field_permissions(
    _auth: Auth<perm::CanReadFieldPermission>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: FilterParams<FieldPermissionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<FieldPermissionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = FieldPermissionService::search(&pagination, &order, &all_filters).await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/field-permissions", post(create_field_permission))
        .route("/field-permissions/{id}", patch(update_field_permission))
        .route("/field-permissions/{id}", delete(delete_field_permission))
        .route("/field-permissions/{id}", get(get_field_permission))
        .route("/field-permissions", get(search_field_permissions))
        .with_state(app_state.clone())
}
