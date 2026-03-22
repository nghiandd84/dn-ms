use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkStr, OkStrResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_fee_model::{
    fee_configuration::{
        FeeConfigurationData, FeeConfigurationForCreateRequest, FeeConfigurationForUpdateRequest,
    },
    state::{FeeAppState, FeeCacheState},
};
use features_fee_service::FeeConfigurationService;

const TAG: &str = "fee";

#[utoipa::path(
    post,
    path = "/fee-configurations",
    tag = TAG,
    request_body = FeeConfigurationForCreateRequest,
    responses(
        (status = 201, description = "Fee configuration created successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_fee_configuration(
    ValidJson(req): ValidJson<FeeConfigurationForCreateRequest>,
) -> Result<ResponseJson<OkStr>> {
    let id = FeeConfigurationService::create_fee_configuration(req).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(id.to_string()),
    }))
}

#[utoipa::path(
    get,
    path = "/fee-configurations/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Fee configuration retrieved successfully", body = FeeConfigurationData),
    )
)]
async fn get_fee_configuration(Path(id): Path<Uuid>) -> Result<ResponseJson<FeeConfigurationData>> {
    let fee_config = FeeConfigurationService::get_fee_configuration_by_id(id).await?;
    Ok(ResponseJson(fee_config))
}

#[utoipa::path(
    get,
    path = "/fee-configurations",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered fee configurations", body = QueryResultResponse<FeeConfigurationData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_fee_configurations(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<FeeConfigurationData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = vec![]; // Add filters if needed
    let result =
        FeeConfigurationService::get_fee_configurations(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/fee-configurations/{id}",
    tag = TAG,
    request_body = FeeConfigurationForUpdateRequest,
    responses(
        (status = 200, description = "Fee configuration updated successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_fee_configuration(
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<FeeConfigurationForUpdateRequest>,
) -> Result<ResponseJson<OkStr>> {
    FeeConfigurationService::update_fee_configuration(id, req).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(id.to_string()),
    }))
}

#[utoipa::path(
    delete,
    path = "/fee-configurations/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Fee configuration deleted successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_fee_configuration(Path(id): Path<Uuid>) -> Result<ResponseJson<OkStr>> {
    FeeConfigurationService::delete_fee_configuration(id).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(id.to_string()),
    }))
}

pub fn routes(app_state: &AppState<FeeAppState, FeeCacheState>) -> Router {
    Router::new()
        .route("/fee-configurations", post(create_fee_configuration))
        .route("/fee-configurations", get(filter_fee_configurations))
        .route("/fee-configurations/{id}", get(get_fee_configuration))
        .route("/fee-configurations/{id}", patch(update_fee_configuration))
        .route("/fee-configurations/{id}", delete(delete_fee_configuration))
        .with_state(app_state.clone())
}
