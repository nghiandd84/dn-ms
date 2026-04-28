use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

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
use shared_shared_extractor::IdempotencyKey;

use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    top_up_transaction::{
        TopUpTransactionData, TopUpTransactionDataFilterParams, TopUpTransactionForCreateRequest,
        TopUpTransactionForUpdateRequest,
    },
};
use shared_shared_auth::permission::Auth;

use crate::permission::{CanCreateTopUp, CanReadTopUp};
use features_wallet_service::TopUpTransactionService;

const TAG: &str = "top_up";

#[utoipa::path(
    post,
    path = "/wallets/{wallet_id}/top-ups",
    tag = TAG,
    request_body = TopUpTransactionForCreateRequest,
    responses(
        (status = 201, description = "Top-up transaction created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_top_up_transaction(
    _auth: Auth<CanCreateTopUp>,
    _idempotency_key: IdempotencyKey,
    Path(wallet_id): Path<Uuid>,
    ValidJson(req): ValidJson<TopUpTransactionForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let top_up_id = TopUpTransactionService::create_top_up_transaction(wallet_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(top_up_id),
    }))
}

#[utoipa::path(
    get,
    path = "/top-ups/{top_up_transaction_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Top-up transaction retrieved successfully", body = TopUpTransactionData),
    )
)]
async fn get_top_up_transaction(
    _auth: Auth<CanReadTopUp>,
    Path(top_up_transaction_id): Path<Uuid>,
) -> Result<ResponseJson<TopUpTransactionData>> {
    let top_up =
        TopUpTransactionService::get_top_up_transaction_by_id(top_up_transaction_id).await?;
    Ok(ResponseJson(top_up))
}

#[utoipa::path(
    get,
    path = "/top-ups",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered top-up transactions", body = QueryResultResponse<TopUpTransactionData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_top_up_transactions(
    _auth: Auth<CanReadTopUp>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TopUpTransactionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TopUpTransactionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        TopUpTransactionService::get_top_up_transactions(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/top-ups/{top_up_transaction_id}",
    tag = TAG,
    request_body = TopUpTransactionForUpdateRequest,
    responses(
        (status = 200, description = "Top-up transaction updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_top_up_transaction(
    _auth: Auth<CanCreateTopUp>,
    _idempotency_key: IdempotencyKey,
    Path(top_up_transaction_id): Path<Uuid>,
    ValidJson(req): ValidJson<TopUpTransactionForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TopUpTransactionService::update_top_up_transaction(top_up_transaction_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(top_up_transaction_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/top-ups/{top_up_transaction_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Top-up transaction deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_top_up_transaction(
    _auth: Auth<CanCreateTopUp>,
    Path(top_up_transaction_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    TopUpTransactionService::delete_top_up_transaction(top_up_transaction_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(top_up_transaction_id),
    }))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/top-ups", post(create_top_up_transaction))
        .route("/top-ups", get(filter_top_up_transactions))
        .route(
            "/top-ups/{top_up_transaction_id}",
            get(get_top_up_transaction),
        )
        .route(
            "/top-ups/{top_up_transaction_id}",
            patch(update_top_up_transaction),
        )
        .route(
            "/top-ups/{top_up_transaction_id}",
            delete(delete_top_up_transaction),
        )
        .with_state(app_state.clone())
}
