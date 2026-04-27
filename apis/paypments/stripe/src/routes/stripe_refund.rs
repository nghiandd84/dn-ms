use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_stripe_model::{
    state::{PaymentsStripeAppState, PaymentsStripeCacheState},
    stripe_refund::{StripeRefundData, StripeRefundForCreateRequest, StripeRefundForUpdateRequest},
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

use features_payments_stripe_service::StripeRefundService;

const TAG: &str = "stripe_refund";

#[utoipa::path(
    post,
    path = "/stripe/refunds",
    tag = TAG,
    request_body = StripeRefundForCreateRequest,
    responses(
        (status = 201, description = "Refund created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_refund(
    ValidJson(req): ValidJson<StripeRefundForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let refund_id = StripeRefundService::create_refund(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(refund_id),
    }))
}

#[utoipa::path(
    get,
    path = "/stripe/refunds/{refund_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Refund retrieved successfully", body = StripeRefundData),
    )
)]
async fn get_refund(Path(refund_id): Path<Uuid>) -> Result<ResponseJson<StripeRefundData>> {
    let refund = StripeRefundService::get_refund_by_id(refund_id).await?;
    Ok(ResponseJson(refund))
}

#[utoipa::path(
    get,
    path = "/stripe/refunds",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered refunds", body = QueryResultResponse<StripeRefundData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_refunds(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<StripeRefundData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]); // TODO: Add filter support
    let result = StripeRefundService::get_refunds(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/stripe/refunds/{refund_id}",
    tag = TAG,
    request_body = StripeRefundForUpdateRequest,
    responses(
        (status = 200, description = "Refund updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_refund(
    Path(refund_id): Path<Uuid>,
    ValidJson(req): ValidJson<StripeRefundForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    StripeRefundService::update_refund(refund_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(refund_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/stripe/refunds/{refund_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Refund deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_refund(Path(refund_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    StripeRefundService::delete_refund(refund_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(refund_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .route("/stripe/refunds", post(create_refund))
        .route("/stripe/refunds", get(filter_refunds))
        .route("/stripe/refunds/{refund_id}", get(get_refund))
        .route("/stripe/refunds/{refund_id}", patch(update_refund))
        .route("/stripe/refunds/{refund_id}", delete(delete_refund))
        .with_state(app_state.clone())
}
