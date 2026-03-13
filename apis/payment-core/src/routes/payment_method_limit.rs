use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_core_model::{
    payment_method_limit::{
        PaymentMethodLimitData, PaymentMethodLimitDataFilterParams,
        PaymentMethodLimitForCreateRequest, PaymentMethodLimitForUpdateRequest,
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

use features_payments_core_service::PaymentMethodLimitService;

const TAG: &str = "payment-method-limit";

#[utoipa::path(
    post,
    path = "/payment-method-limits",
    tag = TAG,
    request_body = PaymentMethodLimitForCreateRequest,
    responses(
        (status = 201, description = "Payment method limit created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_payment_method_limit(
    ValidJson(req): ValidJson<PaymentMethodLimitForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let payment_method_limit_id =
        PaymentMethodLimitService::create_payment_method_limit(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_limit_id),
    }))
}

#[utoipa::path(
    get,
    path = "/payment-method-limits/{payment_method_limit_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment method limit retrieved successfully", body = PaymentMethodLimitData),
    )
)]
pub async fn get_payment_method_limit(
    Path(payment_method_limit_id): Path<Uuid>,
) -> Result<ResponseJson<PaymentMethodLimitData>> {
    let payment_method_limit =
        PaymentMethodLimitService::get_payment_method_limit_by_id(payment_method_limit_id).await?;
    Ok(ResponseJson(payment_method_limit))
}

#[utoipa::path(
    get,
    path = "/payment-method-limits",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered payment method limits", body = QueryResultResponse<PaymentMethodLimitData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_payment_method_limits(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<PaymentMethodLimitDataFilterParams>,
) -> Result<ResponseJson<QueryResult<PaymentMethodLimitData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        PaymentMethodLimitService::get_payment_method_limits(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/payment-method-limits/{payment_method_limit_id}",
    tag = TAG,
    request_body = PaymentMethodLimitForUpdateRequest,
    responses(
        (status = 200, description = "Payment method limit updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_payment_method_limit(
    Path(payment_method_limit_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaymentMethodLimitForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentMethodLimitService::update_payment_method_limit(payment_method_limit_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_limit_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/payment-method-limits/{payment_method_limit_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment method limit deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_payment_method_limit(
    Path(payment_method_limit_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentMethodLimitService::delete_payment_method_limit(payment_method_limit_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_method_limit_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsCoreAppState, PaymentsCoreCacheState>) -> Router {
    Router::new()
        .route("/payment-method-limits", post(create_payment_method_limit))
        .route("/payment-method-limits", get(filter_payment_method_limits))
        .route(
            "/payment-method-limits/{payment_method_limit_id}",
            get(get_payment_method_limit),
        )
        .route(
            "/payment-method-limits/{payment_method_limit_id}",
            patch(update_payment_method_limit),
        )
        .route(
            "/payment-method-limits/{payment_method_limit_id}",
            delete(delete_payment_method_limit),
        )
        .with_state(app_state.clone())
}
