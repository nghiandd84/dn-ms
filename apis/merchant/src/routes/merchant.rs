use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};

use features_merchant_model::{
    merchant::{
        MerchantData, MerchantDataFilterParams, MerchantForCreateRequest, MerchantForUpdateRequest,
    },
    state::{MerchantAppState, MerchantCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkStr, OkStrResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_merchant_service::MerchantService;

const TAG: &str = "merchant";

#[utoipa::path(
    post,
    path = "/merchants",
    tag = TAG,
    request_body = MerchantForCreateRequest,
    responses(
        (status = 201, description = "Merchant created successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_merchant(
    ValidJson(req): ValidJson<MerchantForCreateRequest>,
) -> Result<ResponseJson<OkStr>> {
    let merchant_id = MerchantService::create_merchant(req).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(merchant_id),
    }))
}

#[utoipa::path(
    get,
    path = "/merchants/{merchant_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Merchant retrieved successfully", body = MerchantData),
    )
)]
async fn get_merchant(Path(merchant_id): Path<String>) -> Result<ResponseJson<MerchantData>> {
    let merchant = MerchantService::get_merchant_by_id(merchant_id).await?;
    Ok(ResponseJson(merchant))
}

#[utoipa::path(
    get,
    path = "/merchants",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered merchants", body = QueryResultResponse<MerchantData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_merchants(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<MerchantDataFilterParams>,
) -> Result<ResponseJson<QueryResult<MerchantData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = MerchantService::get_merchants(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/merchants/{merchant_id}",
    tag = TAG,
    request_body = MerchantForUpdateRequest,
    responses(
        (status = 200, description = "Merchant updated successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_merchant(
    Path(merchant_id): Path<String>,
    ValidJson(req): ValidJson<MerchantForUpdateRequest>,
) -> Result<ResponseJson<OkStr>> {
    MerchantService::update_merchant(merchant_id.to_string(), req).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(merchant_id.to_string()),
    }))
}

#[utoipa::path(
    delete,
    path = "/merchants/{merchant_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Merchant deleted successfully", body = OkStrResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_merchant(Path(merchant_id): Path<String>) -> Result<ResponseJson<OkStr>> {
    MerchantService::delete_merchant(merchant_id.to_string()).await?;
    Ok(ResponseJson(OkStr {
        ok: true,
        id: Some(merchant_id.to_string()),
    }))
}

pub fn routes(app_state: &AppState<MerchantAppState, MerchantCacheState>) -> Router {
    Router::new()
        .route("/merchants", post(create_merchant))
        .route("/merchants", get(filter_merchants))
        .route("/merchants/{merchant_id}", get(get_merchant))
        .route("/merchants/{merchant_id}", patch(update_merchant))
        .route("/merchants/{merchant_id}", delete(delete_merchant))
        .with_state(app_state.clone())
}
