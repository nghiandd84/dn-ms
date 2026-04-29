use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_paypal_model::{
    paypal_refund::{PaypalRefundData, PaypalRefundForCreateRequest, PaypalRefundForUpdateRequest},
    state::{PaymentsPaypalAppState, PaymentsPaypalCacheState},
};
use features_payments_paypal_service::PaypalRefundService;

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

use crate::permission::{CanCreateRefund, CanDeleteRefund, CanReadRefund, CanUpdateRefund};

const TAG: &str = "paypal_refund";

#[utoipa::path(post, path = "/refunds", tag = TAG, request_body = PaypalRefundForCreateRequest, responses((status = 201, description = "Refund created", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_refund(
    _auth: Auth<CanCreateRefund>,
    ValidJson(req): ValidJson<PaypalRefundForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = PaypalRefundService::create_refund(req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(id) }))
}

#[utoipa::path(get, path = "/refunds/{refund_id}", tag = TAG, responses((status = 200, description = "Refund retrieved", body = PaypalRefundData)))]
pub async fn get_refund(
    _auth: Auth<CanReadRefund>,
    Path(refund_id): Path<Uuid>,
) -> Result<ResponseJson<PaypalRefundData>> {
    let refund = PaypalRefundService::get_refund_by_id(refund_id).await?;
    Ok(ResponseJson(refund))
}

#[utoipa::path(get, path = "/refunds", tag = TAG, params(Order, Pagination), responses((status = 200, description = "Filtered refunds", body = QueryResultResponse<PaypalRefundData>)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_refunds(
    _auth: Auth<CanReadRefund>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<PaypalRefundData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]);
    let result = PaypalRefundService::get_refunds(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(patch, path = "/refunds/{refund_id}", tag = TAG, request_body = PaypalRefundForUpdateRequest, responses((status = 200, description = "Refund updated", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_refund(
    _auth: Auth<CanUpdateRefund>,
    Path(refund_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaypalRefundForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalRefundService::update_refund(refund_id, req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(refund_id) }))
}

#[utoipa::path(delete, path = "/refunds/{refund_id}", tag = TAG, responses((status = 200, description = "Refund deleted", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_refund(
    _auth: Auth<CanDeleteRefund>,
    Path(refund_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalRefundService::delete_refund(refund_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(refund_id) }))
}

pub fn routes(app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>) -> Router {
    Router::new()
        .route("/refunds", post(create_refund))
        .route("/refunds", get(filter_refunds))
        .route("/refunds/{refund_id}", get(get_refund))
        .route("/refunds/{refund_id}", patch(update_refund))
        .route("/refunds/{refund_id}", delete(delete_refund))
        .with_state(app_state.clone())
}
