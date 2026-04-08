use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_stripe_model::{
    state::{PaymentsStripeAppState, PaymentsStripeCacheState},
    stripe_webhook_event::{
        StripeWebhookEventData, StripeWebhookEventForCreateRequest,
        StripeWebhookEventForUpdateRequest,
    },
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_payments_stripe_service::StripeWebhookEventService;

const TAG: &str = "stripe_webhook_event";

#[utoipa::path(
    post,
    path = "/stripe/webhook-events",
    tag = TAG,
    request_body = StripeWebhookEventForCreateRequest,
    responses(
        (status = 201, description = "Webhook event created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_webhook_event(
    ValidJson(req): ValidJson<StripeWebhookEventForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let webhook_event_id = StripeWebhookEventService::create_webhook_event(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_event_id),
    }))
}

#[utoipa::path(
    get,
    path = "/stripe/webhook-events/{webhook_event_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook event retrieved successfully", body = StripeWebhookEventData),
    )
)]
async fn get_webhook_event(
    Path(webhook_event_id): Path<Uuid>,
) -> Result<ResponseJson<StripeWebhookEventData>> {
    let webhook_event =
        StripeWebhookEventService::get_webhook_event_by_id(webhook_event_id).await?;
    Ok(ResponseJson(webhook_event))
}

#[utoipa::path(
    get,
    path = "/stripe/webhook-events",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered webhook events", body = QueryResultResponse<StripeWebhookEventData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_webhook_events(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<StripeWebhookEventData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = vec![]; // TODO: Add filter support
    let result =
        StripeWebhookEventService::get_webhook_events(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/stripe/webhook-events/{webhook_event_id}",
    tag = TAG,
    request_body = StripeWebhookEventForUpdateRequest,
    responses(
        (status = 200, description = "Webhook event updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_webhook_event(
    Path(webhook_event_id): Path<Uuid>,
    ValidJson(req): ValidJson<StripeWebhookEventForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    StripeWebhookEventService::update_webhook_event(webhook_event_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_event_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/stripe/webhook-events/{webhook_event_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook event deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_webhook_event(Path(webhook_event_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    StripeWebhookEventService::delete_webhook_event(webhook_event_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_event_id),
    }))
}

pub fn routes(app_state: &AppState<PaymentsStripeAppState, PaymentsStripeCacheState>) -> Router {
    Router::new()
        .route("/stripe/webhook-events", post(create_webhook_event))
        .route("/stripe/webhook-events", get(filter_webhook_events))
        .route(
            "/stripe/webhook-events/{webhook_event_id}",
            get(get_webhook_event),
        )
        .route(
            "/stripe/webhook-events/{webhook_event_id}",
            patch(update_webhook_event),
        )
        .route(
            "/stripe/webhook-events/{webhook_event_id}",
            delete(delete_webhook_event),
        )
        .with_state(app_state.clone())
}
