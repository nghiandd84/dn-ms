use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use shared_shared_auth::permission::Auth;

use crate::permission::{CanCreatePayment, CanDeletePayment, CanReadPayment, CanUpdatePayment};

use features_payments_core_model::{
    payment::{
        PaymentData, PaymentDataFilterParams, PaymentForCreateRequest, PaymentForUpdateRequest,
    },
    state::{PaymentsCoreAppState, PaymentsCoreCacheState},
};

use shared_shared_app::{
    event_task::producer::ProducerMessage,
    state::AppState,
};
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
use features_payments_core_stream::{
    PaymentCoreEventMessage, PaymentSucceededMessage, PRODUCER_KEY,
};

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
    _auth: Auth<CanCreatePayment>,
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
pub async fn get_payment(_auth: Auth<CanReadPayment>, Path(payment_id): Path<Uuid>) -> Result<ResponseJson<PaymentData>> {
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
    _auth: Auth<CanReadPayment>,
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
    _auth: Auth<CanUpdatePayment>,
    state: State<AppState<PaymentsCoreAppState, PaymentsCoreCacheState>>,
    Path(payment_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaymentForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let is_succeeded = req.status.as_deref() == Some("succeeded");
    PaymentService::update_payment(payment_id, req).await?;

    if is_succeeded {
        let payment = PaymentService::get_payment_by_id(payment_id).await?;
        let wallet_id = payment
            .metadata
            .as_ref()
            .and_then(|m| m.get("wallet_id"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<Uuid>().ok());
        let producer = state
            .get_producer(PRODUCER_KEY.to_string())
            .expect("Producer not found");
        let message = ProducerMessage {
            key: None,
            payload: PaymentCoreEventMessage::Succeeded {
                message: PaymentSucceededMessage {
                    payment_id,
                    user_id: payment.user_id.unwrap_or_default(),
                    wallet_id,
                    amount: payment.amount.unwrap_or_default(),
                    currency: payment.currency.unwrap_or_default(),
                },
            },
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending payment success event to Kafka: {:?}", e.reason);
        }
    }

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
pub async fn delete_payment(_auth: Auth<CanDeletePayment>, Path(payment_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
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
