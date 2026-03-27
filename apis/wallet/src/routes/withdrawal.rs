use axum::{extract::{Path, Query}, routing::{delete, get, patch, post}, Router};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    withdrawal::{WithdrawalData, WithdrawalDataFilterParams, WithdrawalForCreateRequest, WithdrawalForUpdateRequest},
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

use features_wallet_service::WithdrawalService;

const TAG: &str = "withdrawal";

#[utoipa::path(
    post,
    path = "/wallets/{wallet_id}/withdrawals",
    tag = TAG,
    request_body = WithdrawalForCreateRequest,
    responses(
        (status = 201, description = "Withdrawal created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_withdrawal(
    Path(wallet_id): Path<Uuid>,
    ValidJson(mut req): ValidJson<WithdrawalForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    req.wallet_id = wallet_id;
    let withdrawal_id = WithdrawalService::create_withdrawal(req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(withdrawal_id) }))
}

#[utoipa::path(
    get,
    path = "/withdrawals/{withdrawal_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Withdrawal retrieved", body = WithdrawalData),
    )
)]
async fn get_withdrawal(
    Path(withdrawal_id): Path<Uuid>,
) -> Result<ResponseJson<WithdrawalData>> {
    let withdrawal = WithdrawalService::get_withdrawal_by_id(withdrawal_id).await?;
    Ok(ResponseJson(withdrawal))
}

#[utoipa::path(
    get,
    path = "/withdrawals",
    tag = TAG,
    params(Order, Pagination),
    responses(
        (status = 200, description = "Filtered withdrawals", body = QueryResultResponse<WithdrawalData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_withdrawals(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<WithdrawalDataFilterParams>,
) -> Result<ResponseJson<QueryResult<WithdrawalData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = WithdrawalService::get_withdrawals(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/wallets/{wallet_id}/withdrawals",
    tag = TAG,
    params(Order, Pagination),
    responses(
        (status = 200, description = "Withdrawals for wallet", body = QueryResultResponse<WithdrawalData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_wallet_withdrawals(
    Path(wallet_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<WithdrawalData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = WithdrawalService::get_withdrawals_by_wallet_id(wallet_id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/withdrawals/{withdrawal_id}",
    tag = TAG,
    request_body = WithdrawalForUpdateRequest,
    responses(
        (status = 200, description = "Withdrawal updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_withdrawal(
    Path(withdrawal_id): Path<Uuid>,
    ValidJson(req): ValidJson<WithdrawalForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    WithdrawalService::update_withdrawal(withdrawal_id, req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(withdrawal_id) }))
}

#[utoipa::path(
    delete,
    path = "/withdrawals/{withdrawal_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Withdrawal deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_withdrawal(
    Path(withdrawal_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    WithdrawalService::delete_withdrawal(withdrawal_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(withdrawal_id) }))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/wallets/{wallet_id}/withdrawals", post(create_withdrawal))
        .route("/withdrawals", get(filter_withdrawals))
        .route("/wallets/{wallet_id}/withdrawals", get(get_wallet_withdrawals))
        .route("/withdrawals/{withdrawal_id}", get(get_withdrawal))
        .route("/withdrawals/{withdrawal_id}", patch(update_withdrawal))
        .route("/withdrawals/{withdrawal_id}", delete(delete_withdrawal))
        .with_state(app_state.clone())
}
