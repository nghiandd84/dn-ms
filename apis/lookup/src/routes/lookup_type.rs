use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
    query_params::QueryParams,
};
use shared_shared_extractor::TenantId;

use features_lookup_model::{
    lookup_type::{
        LookupTypeData, LookupTypeDataFilterParams, LookupTypeForCreateRequest,
        LookupTypeForUpdateRequest,
    },
    state::{LookupAppState, LookupCacheState},
};
use features_lookup_service::lookup_type::LookupTypeService;

const TAG: &str = "lookup-type";

#[utoipa::path(
    post,
    path = "/lookup-types",
    tag = TAG,
    request_body = LookupTypeForCreateRequest,
    responses(
        (status = 201, description = "Lookup type created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_lookup_type(
    TenantId(tenant_id): TenantId,
    ValidJson(mut req): ValidJson<LookupTypeForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    debug!("Creating lookup_type for tenant: {}", tenant_id);
    req.tenant_id = Some(tenant_id);
    let id = LookupTypeService::create_lookup_type(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    get,
    path = "/lookup-types",
    tag = TAG,
    params(Pagination, Order),
    responses(
        (status = 200, description = "List of lookup types", body = QueryResultResponse<LookupTypeData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_lookup_types(
    TenantId(tenant_id): TenantId,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<LookupTypeDataFilterParams>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<LookupTypeData>>> {
    debug!("Received request to get lookup_types for tenant_id: {}, pagination: {:?}, order: {:?}, filters: {:?}", tenant_id, query_pagination, query_order, filter_params);
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        LookupTypeService::get_lookup_types(&tenant_id, &filters, &pagination, &order, query_params).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/lookup-types/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Lookup type", body = LookupTypeData),
    )
)]
pub async fn get_lookup_type(
    Path(id): Path<Uuid>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<LookupTypeData>> {
    let result = LookupTypeService::get_lookup_type_by_id(id, query_params).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/lookup-types/{id}",
    tag = TAG,
    request_body = LookupTypeForUpdateRequest,
    responses(
        (status = 200, description = "Lookup type updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_lookup_type(
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<LookupTypeForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    LookupTypeService::update_lookup_type(id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/lookup-types/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Lookup type deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_lookup_type(Path(id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    LookupTypeService::delete_lookup_type(id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<LookupAppState, LookupCacheState>) -> Router {
    Router::new()
        .route("/lookup-types", post(create_lookup_type))
        .route("/lookup-types", get(get_lookup_types))
        .route("/lookup-types/{id}", get(get_lookup_type))
        .route("/lookup-types/{id}", patch(update_lookup_type))
        .route("/lookup-types/{id}", delete(delete_lookup_type))
        .with_state(app_state.clone())
}
