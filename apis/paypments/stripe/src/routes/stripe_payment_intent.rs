use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_stripe_model::{
    state::{PaymentsStripeAppState, PaymentsStripeCacheState},
    stripe_payment_intent::{
        StripePaymentIntentData, StripePaymentIntentForCreateRequest,
        StripePaymentIntentForUpdateRequest,
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

use crate::permission::{
    CanCreatePaymentIntent, CanDeletePaymentIntent, CanReadPaymentIntent, CanUpdatePaymentIntent,
};
use features_payments_stripe_service::StripePaymentIntentService;

const TAG: &str = "stripe_payment_intent";

#[utoipa::path(
    post,
    path = "/stripe/payment-intents",
    tag = TAG,
    request_body = StripePaymentIntentForCreateRequest,
    responses(
        (status = 201, description = "Payment intent created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_payment_intent(
    _auth: Auth<CanCreatePaymentIntent>,
    ValidJson(req): ValidJson<StripePaymentIntentForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let payment_intent_id = StripePaymentIntentService::create_payment_intent(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_intent_id),
    }))
}

#[utoipa::path(
    get,
    path = "/stripe/payment-intents/{payment_intent_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment intent retrieved successfully", body = StripePaymentIntentData),
    )
)]
async fn get_payment_intent(
    _auth: Auth<CanReadPaymentIntent>,
    Path(payment_intent_id): Path<Uuid>,
) -> Result<ResponseJson<StripePaymentIntentData>> {
    let payment_intent =
        StripePaymentIntentService::get_payment_intent_by_id(payment_intent_id).await?;
    Ok(ResponseJson(payment_intent))
}

#[utoipa::path(
    get,
    path = "/stripe/payment-intents",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered payment intents", body = QueryResultResponse<StripePaymentIntentData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_payment_intents(
    _auth: Auth<CanReadPaymentIntent>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<StripePaymentIntentData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]); // TODO: Add filter support
    let result =
        StripePaymentIntentService::get_payment_intents(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/stripe/payment-intents/{payment_intent_id}",
    tag = TAG,
    request_body = StripePaymentIntentForUpdateRequest,
    responses(
        (status = 200, description = "Payment intent updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_payment_intent(
    _auth: Auth<CanUpdatePaymentIntent>,
    Path(payment_intent_id): Path<Uuid>,
    ValidJson(req): ValidJson<StripePaymentIntentForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    StripePaymentIntentService::update_payment_intent(payment_intent_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_intent_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/stripe/payment-intents/{payment_intent_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Payment intent deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_payment_intent(
    _auth: Auth<CanDeletePaymentIntent>,
    Path(payment_intent_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    StripePaymentIntentService::delete_payment_intent(payment_intent_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(payment_intent_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .route("/stripe/payment-intents", post(create_payment_intent))
        .route("/stripe/payment-intents", get(filter_payment_intents))
        .route(
            "/stripe/payment-intents/{payment_intent_id}",
            get(get_payment_intent),
        )
        .route(
            "/stripe/payment-intents/{payment_intent_id}",
            patch(update_payment_intent),
        )
        .route(
            "/stripe/payment-intents/{payment_intent_id}",
            delete(delete_payment_intent),
        )
        .with_state(app_state.clone())
}
