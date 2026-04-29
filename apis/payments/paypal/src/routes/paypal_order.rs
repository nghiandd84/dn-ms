use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_paypal_model::{
    paypal_order::{PaypalOrderData, PaypalOrderForCreateRequest, PaypalOrderForUpdateRequest},
    state::{PaymentsPaypalAppState, PaymentsPaypalCacheState},
};
use features_payments_paypal_service::PaypalOrderService;

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

use crate::permission::{CanCreateOrder, CanDeleteOrder, CanReadOrder, CanUpdateOrder};

const TAG: &str = "paypal_order";

#[utoipa::path(post, path = "/orders", tag = TAG, request_body = PaypalOrderForCreateRequest, responses((status = 201, description = "Order created", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_order(
    _auth: Auth<CanCreateOrder>,
    ValidJson(req): ValidJson<PaypalOrderForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = PaypalOrderService::create_order(req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(id) }))
}

#[utoipa::path(get, path = "/orders/{order_id}", tag = TAG, responses((status = 200, description = "Order retrieved", body = PaypalOrderData)))]
pub async fn get_order(
    _auth: Auth<CanReadOrder>,
    Path(order_id): Path<Uuid>,
) -> Result<ResponseJson<PaypalOrderData>> {
    let order = PaypalOrderService::get_order_by_id(order_id).await?;
    Ok(ResponseJson(order))
}

#[utoipa::path(get, path = "/orders", tag = TAG, params(Order, Pagination), responses((status = 200, description = "Filtered orders", body = QueryResultResponse<PaypalOrderData>)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_orders(
    _auth: Auth<CanReadOrder>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<PaypalOrderData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]);
    let result = PaypalOrderService::get_orders(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(patch, path = "/orders/{order_id}", tag = TAG, request_body = PaypalOrderForUpdateRequest, responses((status = 200, description = "Order updated", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_order(
    _auth: Auth<CanUpdateOrder>,
    Path(order_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaypalOrderForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalOrderService::update_order(order_id, req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(order_id) }))
}

#[utoipa::path(delete, path = "/orders/{order_id}", tag = TAG, responses((status = 200, description = "Order deleted", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_order(
    _auth: Auth<CanDeleteOrder>,
    Path(order_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalOrderService::delete_order(order_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(order_id) }))
}

pub fn routes(app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>) -> Router {
    Router::new()
        .route("/orders", post(create_order))
        .route("/orders", get(filter_orders))
        .route("/orders/{order_id}", get(get_order))
        .route("/orders/{order_id}", patch(update_order))
        .route("/orders/{order_id}", delete(delete_order))
        .with_state(app_state.clone())
}
