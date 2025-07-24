use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Router,
};
use tracing::debug;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkI32, OkI32Response, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};


use features_bakery_model::{order::{
    OrderData, OrderDataFilterParams, OrderDataResponse, OrderForCreateRequest,
}, state::BakeryCacheState};
use features_bakery_service::order::{OrderMutation, OrderQuery};

const TAG: &str = "order";

#[utoipa::path(
    post,
    request_body = OrderForCreateRequest,
    path = "/orders",
    operation_id = "create-order",
    tag = TAG,
    responses(
        (status = 200, description = "Order is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState<BakeryCacheState>>,
    ValidJson(request): ValidJson<OrderForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let role_id = OrderMutation::create(&state.conn, request.into()).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/orders/{order_id}",
    tag = TAG,
    operation_id = "delete-order-by-id",
    responses(
        (status = 200, description = "Order is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(order_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    OrderMutation::delete(&state.conn, order_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/orders/{order_id}",
    tag = TAG,
    operation_id = "get-order-by-id",
    responses(
        (status = 200, description = "Customer Data", body = OrderDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(order_id): Path<i32>,
) -> Result<ResponseJson<OrderData>> {
    let cake = OrderQuery::get_by_id(&state.conn, order_id).await?;
    Ok(ResponseJson(cake))
}

#[utoipa::path(
    get,
    path = "/orders",
    tag = TAG,
    operation_id = "filter-order",
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Filtered Customer", body = QueryResultResponse<OrderData>),
    )
)]
async fn filter(
    state: State<AppState<BakeryCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<OrderDataFilterParams>,
) -> Result<ResponseJson<QueryResult<OrderData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = OrderQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState<BakeryCacheState>) -> Router {
    Router::new()
        .route("/orders", post(create))
        .route("/orders/{order_id}", delete(delete_by_id))
        .route("/orders/{order_id}", get(get_by_id))
        .route("/orders", get(filter))
        .with_state(app_state.clone())
}
