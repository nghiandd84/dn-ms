use axum::{extract::{Path, Query}, routing::{delete, get, patch, post}, Router};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    p2p_transfer::{P2pTransferData, P2pTransferDataFilterParams, P2pTransferForCreateRequest, P2pTransferForUpdateRequest},
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

use features_wallet_service::P2pTransferService;

const TAG: &str = "p2p_transfer";

#[utoipa::path(
    post,
    path = "/p2p-transfers",
    tag = TAG,
    request_body = P2pTransferForCreateRequest,
    responses(
        (status = 201, description = "P2P transfer created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_p2p_transfer(
    ValidJson(req): ValidJson<P2pTransferForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let transfer_id = P2pTransferService::create_p2p_transfer(req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(transfer_id) }))
}

#[utoipa::path(
    get,
    path = "/p2p-transfers/{p2p_transfer_id}",
    tag = TAG,
    responses(
        (status = 200, description = "P2P transfer retrieved", body = P2pTransferData),
    )
)]
async fn get_p2p_transfer(
    Path(p2p_transfer_id): Path<Uuid>,
) -> Result<ResponseJson<P2pTransferData>> {
    let transfer = P2pTransferService::get_p2p_transfer_by_id(p2p_transfer_id).await?;
    Ok(ResponseJson(transfer))
}

#[utoipa::path(
    get,
    path = "/p2p-transfers",
    tag = TAG,
    params(Order, Pagination),
    responses(
        (status = 200, description = "Filtered P2P transfers", body = QueryResultResponse<P2pTransferData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_p2p_transfers(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<P2pTransferDataFilterParams>,
) -> Result<ResponseJson<QueryResult<P2pTransferData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = P2pTransferService::get_p2p_transfers(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/wallets/{wallet_id}/p2p-transfers",
    tag = TAG,
    params(Order, Pagination),
    responses(
        (status = 200, description = "P2P transfers for wallet", body = QueryResultResponse<P2pTransferData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn get_wallet_p2p_transfers(
    Path(wallet_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<P2pTransferData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = P2pTransferService::get_p2p_transfers_by_wallet_id(wallet_id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/p2p-transfers/{p2p_transfer_id}",
    tag = TAG,
    request_body = P2pTransferForUpdateRequest,
    responses(
        (status = 200, description = "P2P transfer updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_p2p_transfer(
    Path(p2p_transfer_id): Path<Uuid>,
    ValidJson(req): ValidJson<P2pTransferForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    P2pTransferService::update_p2p_transfer(p2p_transfer_id, req).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(p2p_transfer_id) }))
}

#[utoipa::path(
    delete,
    path = "/p2p-transfers/{p2p_transfer_id}",
    tag = TAG,
    responses(
        (status = 200, description = "P2P transfer deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_p2p_transfer(
    Path(p2p_transfer_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    P2pTransferService::delete_p2p_transfer(p2p_transfer_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: Some(p2p_transfer_id) }))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/p2p-transfers", post(create_p2p_transfer))
        .route("/p2p-transfers", get(filter_p2p_transfers))
        .route("/wallets/{wallet_id}/p2p-transfers", get(get_wallet_p2p_transfers))
        .route("/p2p-transfers/{p2p_transfer_id}", get(get_p2p_transfer))
        .route("/p2p-transfers/{p2p_transfer_id}", patch(update_p2p_transfer))
        .route("/p2p-transfers/{p2p_transfer_id}", delete(delete_p2p_transfer))
        .with_state(app_state.clone())
}
