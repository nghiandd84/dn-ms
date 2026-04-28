use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_stripe_model::{
    state::{PaymentsStripeAppState, PaymentsStripeCacheState},
    stripe_api_log::{
        StripeApiLogData, StripeApiLogForCreateRequest, StripeApiLogForUpdateRequest,
    },
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use shared_shared_auth::permission::Auth;

use crate::permission::{CanCreateApiLog, CanDeleteApiLog, CanReadApiLog, CanUpdateApiLog};
use features_payments_stripe_service::StripeApiLogService;

const TAG: &str = "stripe_api_log";

#[utoipa::path(
    post,
    path = "/stripe/api-logs",
    tag = TAG,
    request_body = StripeApiLogForCreateRequest,
    responses(
        (status = 201, description = "API log created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_api_log(
    _auth: Auth<CanCreateApiLog>,
    ValidJson(req): ValidJson<StripeApiLogForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let api_log_id = StripeApiLogService::create_api_log(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(api_log_id),
    }))
}

#[utoipa::path(
    get,
    path = "/stripe/api-logs/{api_log_id}",
    tag = TAG,
    responses(
        (status = 200, description = "API log retrieved successfully", body = StripeApiLogData),
    )
)]
async fn get_api_log(_auth: Auth<CanReadApiLog>, Path(api_log_id): Path<Uuid>) -> Result<ResponseJson<StripeApiLogData>> {
    let api_log = StripeApiLogService::get_api_log_by_id(api_log_id).await?;
    Ok(ResponseJson(api_log))
}

#[utoipa::path(
    get,
    path = "/stripe/api-logs",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered API logs", body = QueryResultResponse<StripeApiLogData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_api_logs(
    _auth: Auth<CanReadApiLog>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<StripeApiLogData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]); // TODO: Add filter support
    let result = StripeApiLogService::get_api_logs(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/stripe/api-logs/{api_log_id}",
    tag = TAG,
    request_body = StripeApiLogForUpdateRequest,
    responses(
        (status = 200, description = "API log updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_api_log(
    _auth: Auth<CanUpdateApiLog>,
    Path(api_log_id): Path<Uuid>,
    ValidJson(req): ValidJson<StripeApiLogForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    StripeApiLogService::update_api_log(api_log_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(api_log_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/stripe/api-logs/{api_log_id}",
    tag = TAG,
    responses(
        (status = 200, description = "API log deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_api_log(_auth: Auth<CanDeleteApiLog>, Path(api_log_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    StripeApiLogService::delete_api_log(api_log_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(api_log_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .route("/stripe/api-logs", post(create_api_log))
        .route("/stripe/api-logs", get(filter_api_logs))
        .route("/stripe/api-logs/{api_log_id}", get(get_api_log))
        .route("/stripe/api-logs/{api_log_id}", patch(update_api_log))
        .route("/stripe/api-logs/{api_log_id}", delete(delete_api_log))
        .with_state(app_state.clone())
}
