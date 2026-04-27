use axum::{
    extract::{Path, Query},
    routing::{delete, get, post, put},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};

use features_wallet_model::{
    idempotency::{
        IdempotencyKeyData, IdempotencyKeyForCreateRequest, IdempotencyKeyForUpdateRequest,
    },
    state::{WalletAppState, WalletCacheState},
};
use features_wallet_service::IdempotencyService;

const TAG: &str = "idempotency";

#[utoipa::path(
    post,
    path = "/idempotency-keys",
    tag = TAG,
    request_body = IdempotencyKeyForCreateRequest,
    responses(
        (status = 201, description = "Idempotency key created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_idempotency_key(
    ValidJson(req): ValidJson<IdempotencyKeyForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let key_id = IdempotencyService::create_idempotency_key(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

#[utoipa::path(
    get,
    path = "/idempotency-keys/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Idempotency key retrieved", body = IdempotencyKeyData),
    )
)]
async fn get_idempotency_key_by_id(
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<IdempotencyKeyData>> {
    let idempotency_key = IdempotencyService::get_idempotency_key_by_id(id).await?;
    Ok(ResponseJson(idempotency_key))
}

#[utoipa::path(
    get,
    path = "/idempotency-keys/key/{key}",
    tag = TAG,
    responses(
        (status = 200, description = "Idempotency key retrieved by key", body = IdempotencyKeyData),
    )
)]
async fn get_idempotency_key_by_key(
    Path(key): Path<String>,
) -> Result<ResponseJson<IdempotencyKeyData>> {
    let idempotency_key = IdempotencyService::get_idempotency_key_by_key(&key).await?;
    Ok(ResponseJson(idempotency_key))
}

#[utoipa::path(
    get,
    path = "/idempotency-keys",
    tag = TAG,
    params(Order, Pagination),
    responses(
        (status = 200, description = "Filtered idempotency keys", body = QueryResult<IdempotencyKeyData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_idempotency_keys(
    Query(pagination): Query<Pagination>,
    Query(order): Query<Order>,
) -> Result<ResponseJson<QueryResult<IdempotencyKeyData>>> {
    let result = IdempotencyService::get_idempotency_keys(
        &pagination,
        &order,
        &FilterCondition::And(vec![]),
    )
    .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    put,
    path = "/idempotency-keys/{id}",
    tag = TAG,
    request_body = IdempotencyKeyForUpdateRequest,
    responses(
        (status = 200, description = "Idempotency key updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_idempotency_key(
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<IdempotencyKeyForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    IdempotencyService::update_idempotency_key(id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/idempotency-keys/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Idempotency key deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_idempotency_key(Path(id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    IdempotencyService::delete_idempotency_key(id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/idempotency-keys", post(create_idempotency_key))
        .route("/idempotency-keys", get(get_idempotency_keys))
        .route("/idempotency-keys/{id}", get(get_idempotency_key_by_id))
        .route("/idempotency-keys/{id}", put(update_idempotency_key))
        .route("/idempotency-keys/{id}", delete(delete_idempotency_key))
        .route(
            "/idempotency-keys/key/{key}",
            get(get_idempotency_key_by_key),
        )
        .with_state(app_state.clone())
}
