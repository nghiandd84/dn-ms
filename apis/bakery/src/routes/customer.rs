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


use features_bakery_model::{customer::{
    CustomerData, CustomerDataFilterParams, CustomerDataResponse, CustomerForCreateRequest,
}, state::BakeryCacheState};
use features_bakery_service::customer::{CustomerMutation, CustomerQuery};

const TAG: &str = "customer";

#[utoipa::path(
    post,
    request_body = CustomerForCreateRequest,
    path = "/customers",
    operation_id = "create-customer",
    tag = TAG,
    responses(
        (status = 200, description = "Customer is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState<BakeryCacheState>>,
    ValidJson(request): ValidJson<CustomerForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let role_id = CustomerMutation::create(&state.conn, request.into()).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/customers/{customer_id}",
    tag = TAG,
    operation_id = "delete-customer-by-id",
    responses(
        (status = 200, description = "Customer is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(customer_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    CustomerMutation::delete(&state.conn, customer_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/customers/{customer_id}",
    tag = TAG,
    operation_id = "get-customer-by-id",
    responses(
        (status = 200, description = "Customer Data", body = CustomerDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(customer_id): Path<i32>,
) -> Result<ResponseJson<CustomerData>> {
    let cake = CustomerQuery::get_by_id(&state.conn, customer_id).await?;
    Ok(ResponseJson(cake))
}

#[utoipa::path(
    get,
    path = "/customers",
    tag = TAG,
    operation_id = "filter-customers",
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Filtered Customer", body = QueryResultResponse<CustomerData>),
    )
)]
async fn filter(
    state: State<AppState<BakeryCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<CustomerDataFilterParams>,
) -> Result<ResponseJson<QueryResult<CustomerData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = CustomerQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState<BakeryCacheState>) -> Router {
    Router::new()
        .route("/customers", post(create))
        .route("/customers/{customer_id}", delete(delete_by_id))
        .route("/customers/{customer_id}", get(get_by_id))
        .route("/customers", get(filter))
        .with_state(app_state.clone())
}
