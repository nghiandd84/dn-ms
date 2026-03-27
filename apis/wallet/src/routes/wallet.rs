use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    wallet::{WalletData, WalletDataFilterParams, WalletForCreateRequest, WalletForUpdateRequest},
    TopUpTransactionData, TransactionData,
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

use features_wallet_service::{TopUpTransactionService, TransactionService, WalletService};

const TAG: &str = "wallet";

#[utoipa::path(
    post,
    path = "/wallets",
    tag = TAG,
    request_body = WalletForCreateRequest,
    responses(
        (status = 201, description = "Wallet created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_wallet(
    ValidJson(mut req): ValidJson<WalletForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    // Get user_id from auth context
    let user_id = Uuid::nil();
    req.user_id = Some(user_id);
    let wallet_id = WalletService::create_wallet(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(wallet_id),
    }))
}

#[utoipa::path(
    get,
    path = "/wallets/{wallet_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Wallet retrieved successfully", body = WalletData),
    )
)]
async fn get_wallet(Path(wallet_id): Path<Uuid>) -> Result<ResponseJson<WalletData>> {
    let wallet = WalletService::get_wallet_by_id(wallet_id).await?;
    Ok(ResponseJson(wallet))
}

#[utoipa::path(
    get,
    path = "/wallets/user/{user_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Wallet retrieved successfully", body = WalletData),
    )
)]
async fn get_wallet_by_user(
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<QueryResult<WalletData>>> {
    let wallet = WalletService::get_wallet_by_user_id(user_id).await?;
    Ok(ResponseJson(wallet))
}

#[utoipa::path(
    get,
    path = "/wallets",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered wallets", body = QueryResultResponse<WalletData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_wallets(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<WalletDataFilterParams>,
) -> Result<ResponseJson<QueryResult<WalletData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = WalletService::get_wallets(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/wallets/{wallet_id}",
    tag = TAG,
    request_body = WalletForUpdateRequest,
    responses(
        (status = 200, description = "Wallet updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_wallet(
    Path(wallet_id): Path<Uuid>,
    ValidJson(req): ValidJson<WalletForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    WalletService::update_wallet(wallet_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(wallet_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/wallets/{wallet_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Wallet deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_wallet(Path(wallet_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    WalletService::delete_wallet(wallet_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(wallet_id),
    }))
}

#[utoipa::path(
    get,
    path = "/wallets/{wallet_id}/transactions",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Wallet transactions", body = QueryResultResponse<TransactionData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_wallet_transactions(
    Path(wallet_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<TransactionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result =
        TransactionService::get_transactions_by_wallet_id(wallet_id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/wallets/{wallet_id}/top-ups",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Wallet top-up transactions", body = QueryResultResponse<TopUpTransactionData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_wallet_top_up_transactions(
    Path(wallet_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<TopUpTransactionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = TopUpTransactionService::get_top_up_transactions_by_wallet_id(
        wallet_id,
        &pagination,
        &order,
    )
    .await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/wallets", post(create_wallet))
        .route("/wallets", get(filter_wallets))
        .route(
            "/wallets/{wallet_id}/top-ups",
            get(get_wallet_top_up_transactions),
        )
        .route(
            "/wallets/{wallet_id}/transactions",
            get(get_wallet_transactions),
        )
        .route("/wallets/{wallet_id}", get(get_wallet))
        .route("/wallets/user/{user_id}", get(get_wallet_by_user))
        .route("/wallets/{wallet_id}", patch(update_wallet))
        .route("/wallets/{wallet_id}", delete(delete_wallet))
        .with_state(app_state.clone())
}
