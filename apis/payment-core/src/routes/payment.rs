use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_core_model::{
    payment::{
        PaymentData, PaymentDataFilterParams, PaymentForCreateRequest, PaymentForUpdateRequest,
    },
    state::{PaymentsCoreAppState, PaymentsCoreCacheState},
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_payments_core_service::PaymentService;

const TAG: &str = "payment";

#[utoipa::path(
    post,
    path = "/payments",
    tag = TAG,
    request_body = PaymentForCreateRequest,
    responses(
        (status = 201, description = "Payment created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_payment(
    ValidJson(req): ValidJson<PaymentForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let payment_id = PaymentService::create_payment(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_id),
    }))
}

#[utoipa::path(
    get,
    path = "/payments/{payment_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment retrieved successfully", body = PaymentData),
    )
)]
pub async fn get_payment(Path(payment_id): Path<Uuid>) -> Result<ResponseJson<PaymentData>> {
    let payment = PaymentService::get_payment_by_id(payment_id).await?;
    Ok(ResponseJson(payment))
}

#[utoipa::path(
    get,
    path = "/payments",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered payments", body = QueryResultResponse<PaymentData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_payments(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<PaymentDataFilterParams>,
) -> Result<ResponseJson<QueryResult<PaymentData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = PaymentService::get_payments(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/payments/{payment_id}",
    tag = TAG,
    request_body = PaymentForUpdateRequest,
    responses(
        (status = 200, description = "Payment updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_payment(
    Path(payment_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaymentForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentService::update_payment(payment_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/payments/{payment_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_payment(Path(payment_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    PaymentService::delete_payment(payment_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsCoreAppState, PaymentsCoreCacheState>) -> Router {
    Router::new()
        .route("/payments", post(create_payment))
        .route("/payments", get(filter_payments))
        .route("/payments/{payment_id}", get(get_payment))
        .route("/payments/{payment_id}", patch(update_payment))
        .route("/payments/{payment_id}", delete(delete_payment))
        .with_state(app_state.clone())
}
