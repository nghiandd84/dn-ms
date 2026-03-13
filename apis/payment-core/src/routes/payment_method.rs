use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_core_model::{
    payment_method::{
        PaymentMethodData, PaymentMethodDataFilterParams, PaymentMethodForCreateRequest,
        PaymentMethodForUpdateRequest,
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

use features_payments_core_service::PaymentMethodService;

const TAG: &str = "payment-method";

#[utoipa::path(
    post,
    path = "/payment-methods",
    tag = TAG,
    request_body = PaymentMethodForCreateRequest,
    responses(
        (status = 201, description = "Payment method created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_payment_method(
    ValidJson(req): ValidJson<PaymentMethodForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let payment_method_id = PaymentMethodService::create_payment_method(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_id),
    }))
}

#[utoipa::path(
    get,
    path = "/payment-methods/{payment_method_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment method retrieved successfully", body = PaymentMethodData),
    )
)]
pub async fn get_payment_method(
    Path(payment_method_id): Path<Uuid>,
) -> Result<ResponseJson<PaymentMethodData>> {
    let payment_method = PaymentMethodService::get_payment_method_by_id(payment_method_id).await?;
    Ok(ResponseJson(payment_method))
}

#[utoipa::path(
    get,
    path = "/payment-methods",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered payment methods", body = QueryResultResponse<PaymentMethodData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_payment_methods(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<PaymentMethodDataFilterParams>,
) -> Result<ResponseJson<QueryResult<PaymentMethodData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = PaymentMethodService::get_payment_methods(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/payment-methods/{payment_method_id}",
    tag = TAG,
    request_body = PaymentMethodForUpdateRequest,
    responses(
        (status = 200, description = "Payment method updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_payment_method(
    Path(payment_method_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaymentMethodForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentMethodService::update_payment_method(payment_method_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/payment-methods/{payment_method_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment method deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_payment_method(
    Path(payment_method_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentMethodService::delete_payment_method(payment_method_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsCoreAppState, PaymentsCoreCacheState>) -> Router {
    Router::new()
        .route("/payment-methods", post(create_payment_method))
        .route("/payment-methods", get(filter_payment_methods))
        .route(
            "/payment-methods/{payment_method_id}",
            get(get_payment_method),
        )
        .route(
            "/payment-methods/{payment_method_id}",
            patch(update_payment_method),
        )
        .route(
            "/payment-methods/{payment_method_id}",
            delete(delete_payment_method),
        )
        .with_state(app_state.clone())
}
