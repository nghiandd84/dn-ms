use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::instrument;
use uuid::Uuid;

use features_merchant_model::state::{MerchantAppState, MerchantCacheState};
use features_merchant_model::webhook::{
    WebhookData, WebhookDataFilterParams, WebhookForCreateRequest, WebhookForUpdateRequest,
};
use features_merchant_service::WebhookService;

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

const TAG: &str = "webhook";

#[utoipa::path(
    post,
    path = "/webhooks",
    tag = TAG,
    request_body = WebhookForCreateRequest,
    responses(
        (status = 200, description = "Webhook created", body = OkUuidResponse),
    )
)]
#[instrument(skip_all)]
async fn create_webhook(
    ValidJson(req): ValidJson<WebhookForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = WebhookService::create_webhook(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    get,
    path = "/webhooks/{webhook_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook retrieved", body = WebhookData),
    )
)]
async fn get_webhook(Path(webhook_id): Path<Uuid>) -> Result<ResponseJson<WebhookData>> {
    let item = WebhookService::get_webhook_by_id(webhook_id).await?;
    Ok(ResponseJson(item))
}

#[utoipa::path(
    get,
    path = "/webhooks",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered webhooks", body = QueryResultResponse<WebhookData>),
    )
)]
async fn filter_webhooks(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<WebhookDataFilterParams>,
) -> Result<ResponseJson<QueryResult<WebhookData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = WebhookService::get_webhooks(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/webhooks/{webhook_id}",
    tag = TAG,
    request_body = WebhookForUpdateRequest,
    responses(
        (status = 200, description = "Webhook updated", body = OkUuidResponse),
    )
)]
#[instrument(skip_all)]
async fn update_webhook(
    Path(webhook_id): Path<Uuid>,
    ValidJson(req): ValidJson<WebhookForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    WebhookService::update_webhook(webhook_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/webhooks/{webhook_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Webhook deleted", body = OkUuidResponse),
    )
)]
#[instrument(skip_all)]
async fn delete_webhook(Path(webhook_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    WebhookService::delete_webhook(webhook_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(webhook_id),
    }))
}

pub fn routes(app_state: &AppState<MerchantAppState, MerchantCacheState>) -> Router {
    Router::new()
        .route("/webhooks", post(create_webhook))
        .route("/webhooks", get(filter_webhooks))
        .route("/webhooks/{webhook_id}", get(get_webhook))
        .route("/webhooks/{webhook_id}", patch(update_webhook))
        .route("/webhooks/{webhook_id}", delete(delete_webhook))
        .with_state(app_state.clone())
}
