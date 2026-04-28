use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use shared_shared_auth::permission::Auth;

use crate::permission::{CanCreateAttempt, CanDeleteAttempt, CanReadAttempt, CanUpdateAttempt};

use features_payments_core_model::{
    payment_attempt::{
        PaymentAttemptData, PaymentAttemptDataFilterParams, PaymentAttemptForCreateRequest,
        PaymentAttemptForUpdateRequest,
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

use features_payments_core_service::PaymentAttemptService;

const TAG: &str = "payment-attempt";

#[utoipa::path(
    post,
    path = "/payment-attempts",
    tag = TAG,
    request_body = PaymentAttemptForCreateRequest,
    responses(
        (status = 201, description = "Payment attempt created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_payment_attempt(
    _auth: Auth<CanCreateAttempt>,
    ValidJson(req): ValidJson<PaymentAttemptForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let payment_attempt_id = PaymentAttemptService::create_payment_attempt(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_attempt_id),
    }))
}

#[utoipa::path(
    get,
    path = "/payment-attempts/{payment_attempt_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment attempt retrieved successfully", body = PaymentAttemptData),
    )
)]
pub async fn get_payment_attempt(
    _auth: Auth<CanReadAttempt>,
    Path(payment_attempt_id): Path<Uuid>,
) -> Result<ResponseJson<PaymentAttemptData>> {
    let payment_attempt =
        PaymentAttemptService::get_payment_attempt_by_id(payment_attempt_id).await?;
    Ok(ResponseJson(payment_attempt))
}

#[utoipa::path(
    get,
    path = "/payment-attempts",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered payment attempts", body = QueryResultResponse<PaymentAttemptData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_payment_attempts(
    _auth: Auth<CanReadAttempt>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<PaymentAttemptDataFilterParams>,
) -> Result<ResponseJson<QueryResult<PaymentAttemptData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = PaymentAttemptService::get_payment_attempts(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/payment-attempts/{payment_attempt_id}",
    tag = TAG,
    request_body = PaymentAttemptForUpdateRequest,
    responses(
        (status = 200, description = "Payment attempt updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_payment_attempt(
    _auth: Auth<CanUpdateAttempt>,
    Path(payment_attempt_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaymentAttemptForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentAttemptService::update_payment_attempt(payment_attempt_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_attempt_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/payment-attempts/{payment_attempt_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment attempt deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_payment_attempt(
    _auth: Auth<CanDeleteAttempt>,
    Path(payment_attempt_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaymentAttemptService::delete_payment_attempt(payment_attempt_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_attempt_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsCoreAppState, PaymentsCoreCacheState>) -> Router {
    Router::new()
        .route("/payment-attempts", post(create_payment_attempt))
        .route("/payment-attempts", get(filter_payment_attempts))
        .route(
            "/payment-attempts/{payment_attempt_id}",
            get(get_payment_attempt),
        )
        .route(
            "/payment-attempts/{payment_attempt_id}",
            patch(update_payment_attempt),
        )
        .route(
            "/payment-attempts/{payment_attempt_id}",
            delete(delete_payment_attempt),
        )
        .with_state(app_state.clone())
}
