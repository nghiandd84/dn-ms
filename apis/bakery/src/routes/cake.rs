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

use features_bakery_model::cake::{
    CakeData, CakeDataFilterParams, CakeDataResponse, CakeForCreateRequest,
};
use features_bakery_service::cake::{CakeMutation, CakeQuery};

const TAG: &str = "cake";

#[utoipa::path(
    post,
    request_body = CakeForCreateRequest,
    path = "/cakes",
    tag = TAG,
    operation_id = "create-cake",
    responses(
        (status = 200, description = "Cake is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState>,
    ValidJson(request): ValidJson<CakeForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let role_id = CakeMutation::create(&state.conn,  request.into()).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/cakes/{cake_id}",
    tag = TAG,
    operation_id = "delete-cake-by-id",
    responses(
        (status = 200, description = "Cake is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState>,
    Path(cake_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    CakeMutation::delete(&state.conn, cake_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/cakes/{cake_id}",
    tag = TAG,
    operation_id = "get-cake-by-id",
    responses(
        (status = 200, description = "Cake Data", body = CakeDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState>,
    Path(cake_id): Path<i32>,
) -> Result<ResponseJson<CakeData>> {
    let cake = CakeQuery::get_by_id(&state.conn, cake_id).await?;
    Ok(ResponseJson(cake))
}

#[utoipa::path(
    get,
    path = "/cakes",
    tag = TAG,
    operation_id = "filter-cakes",
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Filtered Cake", body = QueryResultResponse<CakeData>),
    )
)]
async fn filter(
    state: State<AppState>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<CakeDataFilterParams>,
) -> Result<ResponseJson<QueryResult<CakeData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = CakeQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/cakes", post(create))
        .route("/cakes/{cake_id}", delete(delete_by_id))
        .route("/cakes/{cake_id}", get(get_by_id))
        .route("/cakes", get(filter))
        .with_state(app_state.clone())
}
