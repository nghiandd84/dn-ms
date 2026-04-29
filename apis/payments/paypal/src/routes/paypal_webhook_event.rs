use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_payments_paypal_model::{
    paypal_webhook_event::{
        PaypalWebhookEventData, PaypalWebhookEventForCreateRequest,
        PaypalWebhookEventForUpdateRequest,
    },
    state::{PaymentsPaypalAppState, PaymentsPaypalCacheState},
};
use features_payments_paypal_service::PaypalWebhookEventService;

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

use crate::permission::{
    CanCreateWebhookEvent, CanDeleteWebhookEvent, CanReadWebhookEvent, CanUpdateWebhookEvent,
};

const TAG: &str = "paypal_webhook_event";

#[utoipa::path(post, path = "/webhook-events", tag = TAG, request_body = PaypalWebhookEventForCreateRequest, responses((status = 201, description = "Webhook event created", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_webhook_event(
    _auth: Auth<CanCreateWebhookEvent>,
    ValidJson(req): ValidJson<PaypalWebhookEventForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = PaypalWebhookEventService::create_webhook_event(req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(id) }))
}

#[utoipa::path(get, path = "/webhook-events/{webhook_event_id}", tag = TAG, responses((status = 200, description = "Webhook event retrieved", body = PaypalWebhookEventData)))]
pub async fn get_webhook_event(
    _auth: Auth<CanReadWebhookEvent>,
    Path(webhook_event_id): Path<Uuid>,
) -> Result<ResponseJson<PaypalWebhookEventData>> {
    let event = PaypalWebhookEventService::get_webhook_event_by_id(webhook_event_id).await?;
    Ok(ResponseJson(event))
}

#[utoipa::path(get, path = "/webhook-events", tag = TAG, params(Order, Pagination), responses((status = 200, description = "Filtered webhook events", body = QueryResultResponse<PaypalWebhookEventData>)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn filter_webhook_events(
    _auth: Auth<CanReadWebhookEvent>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<PaypalWebhookEventData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = FilterCondition::And(vec![]);
    let result =
        PaypalWebhookEventService::get_webhook_events(&filters, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(patch, path = "/webhook-events/{webhook_event_id}", tag = TAG, request_body = PaypalWebhookEventForUpdateRequest, responses((status = 200, description = "Webhook event updated", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_webhook_event(
    _auth: Auth<CanUpdateWebhookEvent>,
    Path(webhook_event_id): Path<Uuid>,
    ValidJson(req): ValidJson<PaypalWebhookEventForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalWebhookEventService::update_webhook_event(webhook_event_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_event_id),
    }))
}

#[utoipa::path(delete, path = "/webhook-events/{webhook_event_id}", tag = TAG, responses((status = 200, description = "Webhook event deleted", body = OkUuidResponse)))]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_webhook_event(
    _auth: Auth<CanDeleteWebhookEvent>,
    Path(webhook_event_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    PaypalWebhookEventService::delete_webhook_event(webhook_event_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_event_id),
    }))
}

pub fn routes(
    app_state: &AppState<PaymentsPaypalAppState, PaymentsPaypalCacheState>,
) -> Router {
    Router::new()
        .route("/webhook-events", post(create_webhook_event))
        .route("/webhook-events", get(filter_webhook_events))
        .route(
            "/webhook-events/{webhook_event_id}",
            get(get_webhook_event),
        )
        .route(
            "/webhook-events/{webhook_event_id}",
            patch(update_webhook_event),
        )
        .route(
            "/webhook-events/{webhook_event_id}",
            delete(delete_webhook_event),
        )
        .with_state(app_state.clone())
}
