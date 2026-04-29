use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_paypal_model::{
    paypal_api_log::{
        PaypalApiLogData, PaypalApiLogForCreateRequest, PaypalApiLogForUpdateRequest,
    },
    state::{PaymentsPaypalAppState, PaymentsPaypalCacheState},
};
use features_payments_paypal_service::PaypalApiLogService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use crate::permission::{CanCreateApiLog, CanDeleteApiLog, CanReadApiLog, CanUpdateApiLog};

const TAG: &str = "paypal_api_log";

#[utoipa::path(post, path = "/api-logs", tag = TAG, request_body = PaypalApiLogForCreateRequest, responses((status = 201, description = "API log created", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_api_log(
    _auth: Auth<CanCreateApiLog>,
    ValidJson(req): ValidJson<PaypalApiLogForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = PaypalApiLogService::create_api_log(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(get, path = "/api-logs/{api_log_id}", tag = TAG, responses((status = 200, description = "API log retrieved", body = PaypalApiLogData)))]
pub async fn get_api_log(
    _auth: Auth<CanReadApiLog>,
    Path(api_log_id): Path<Uuid>,
) -> Result<ResponseJson<PaypalApiLogData>> {
    let log = PaypalApiLogService::get_api_log_by_id(api_log_id).await?;
    Ok(ResponseJson(log))
}

#[utoipa::path(get, path = "/api-logs", tag = TAG, params(Order, Pagination), responses((status = 200, description = "Filtered API logs", body = QueryResultResponse<PaypalApiLogData>)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_api_logs(
    _auth: Auth<CanReadApiLog>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<PaypalApiLogData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]);
    let result = PaypalApiLogService::get_api_logs(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(patch, path = "/api-logs/{api_log_id}", tag = TAG, request_body = PaypalApiLogForUpdateRequest, responses((status = 200, description = "API log updated", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_api_log(
    _auth: Auth<CanUpdateApiLog>,
    Path(api_log_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaypalApiLogForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalApiLogService::update_api_log(api_log_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(api_log_id),
    }))
}

#[utoipa::path(delete, path = "/api-logs/{api_log_id}", tag = TAG, responses((status = 200, description = "API log deleted", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_api_log(
    _auth: Auth<CanDeleteApiLog>,
    Path(api_log_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalApiLogService::delete_api_log(api_log_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(api_log_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>) -> Router {
    Router::new()
        .route("/api-logs", post(create_api_log))
        .route("/api-logs", get(filter_api_logs))
        .route("/api-logs/{api_log_id}", get(get_api_log))
        .route("/api-logs/{api_log_id}", patch(update_api_log))
        .route("/api-logs/{api_log_id}", delete(delete_api_log))
        .with_state(app_state.clone())
}
