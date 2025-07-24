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


use features_bakery_entities::bakery::BakeryForCreateDto;
use features_bakery_model::{bakery::{
    BakeryData, BakeryDataFilterParams, BakeryDataResponse, BakeryForCreateRequest,
}, state::BakeryCacheState};
use features_bakery_service::bakery::{BakeryMutation, BakeryQuery};

const TAG: &str = "bakery";

#[utoipa::path(
    post,
    request_body = BakeryForCreateRequest,
    path = "/bakeries",
    tag = TAG,
    operation_id = "create-bakery",
    responses(
        (status = 200, description = "Bakery is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState<BakeryCacheState>>,
    ValidJson(request): ValidJson<BakeryForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let dto: BakeryForCreateDto = request.into();
    debug!("create_baker: {:?}", dto);
    let role_id = BakeryMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/bakeries/{baker_id}",
    tag = TAG,
    operation_id = "delete-bakery-by-id",
    responses(
        (status = 200, description = "Baker is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(baker_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    BakeryMutation::delete(&state.conn, baker_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/bakeries/{baker_id}",
    tag = TAG,
    operation_id = "get-bakery-by-id",
    responses(
        (status = 200, description = "Baker Data", body = BakeryDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(baker_id): Path<i32>,
) -> Result<ResponseJson<BakeryData>> {
    let baker = BakeryQuery::get_by_id(&state.conn, baker_id).await?;
    Ok(ResponseJson(baker))
}

#[utoipa::path(
    get,
    path = "/bakeries",
    tag = TAG,
    operation_id = "filter-bakeries",
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Filtered Baker", body = QueryResultResponse<BakeryData>),
    )
)]
async fn filter(
    state: State<AppState<BakeryCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<BakeryDataFilterParams>,
) -> Result<ResponseJson<QueryResult<BakeryData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = BakeryQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState<BakeryCacheState>) -> Router {
    Router::new()
        .route("/bakeries", post(create))
        .route("/bakeries/{baker_id}", delete(delete_by_id))
        .route("/bakeries/{baker_id}", get(get_by_id))
        .route("/bakeries", get(filter))
        .with_state(app_state.clone())
}
