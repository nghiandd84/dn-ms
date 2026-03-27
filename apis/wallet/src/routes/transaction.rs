use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_wallet_model::{
    state::{WalletAppState, WalletCacheState},
    transaction::{
        TransactionData, TransactionDataFilterParams, TransactionForCreateRequest,
        TransactionForUpdateRequest,
    },
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

use features_wallet_service::TransactionService;

const TAG: &str = "transaction";

#[utoipa::path(
    post,
    path = "/transactions",
    tag = TAG,
    request_body = TransactionForCreateRequest,
    responses(
        (status = 201, description = "Transaction created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_transaction(
    ValidJson(req): ValidJson<TransactionForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let transaction_id = TransactionService::create_transaction(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(transaction_id),
    }))
}

#[utoipa::path(
    get,
    path = "/transactions/{transaction_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Transaction retrieved successfully", body = TransactionData),
    )
)]
async fn get_transaction(
    Path(transaction_id): Path<Uuid>,
) -> Result<ResponseJson<TransactionData>> {
    let transaction = TransactionService::get_transaction_by_id(transaction_id).await?;
    Ok(ResponseJson(transaction))
}

#[utoipa::path(
    get,
    path = "/transactions",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered transactions", body = QueryResultResponse<TransactionData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_transactions(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TransactionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TransactionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = TransactionService::get_transactions(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/transactions/{transaction_id}",
    tag = TAG,
    request_body = TransactionForUpdateRequest,
    responses(
        (status = 200, description = "Transaction updated successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn update_transaction(
    Path(transaction_id): Path<Uuid>,
    ValidJson(req): ValidJson<TransactionForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TransactionService::update_transaction(transaction_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(transaction_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/transactions/{transaction_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Transaction deleted successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn delete_transaction(Path(transaction_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    TransactionService::delete_transaction(transaction_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(transaction_id),
    }))
}

pub fn routes(app_state: &AppState<WalletAppState, WalletCacheState>) -> Router {
    Router::new()
        .route("/transactions", post(create_transaction))
        .route("/transactions", get(filter_transactions))
        .route("/transactions/{transaction_id}", get(get_transaction))
        .route("/transactions/{transaction_id}", patch(update_transaction))
        .route("/transactions/{transaction_id}", delete(delete_transaction))
        .with_state(app_state.clone())
}
