use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Router,
};
use tracing::debug;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkI32, OkI32Response, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_bakery_entities::baker::BakerForCreateDto;
use features_bakery_model::baker::{
    BakerData, BakerDataFilterParams, BakerDataResponse, BakerForCreateRequest,
};
use features_bakery_service::baker::{BakerMutation, BakerQuery};

const TAG: &str = "baker";

#[utoipa::path(
    post,
    request_body = BakerForCreateRequest,
    path = "/bakers",
    operation_id = "create-baker",
    tag = TAG,
    responses(
        (status = 200, description = "Baker is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState>,
    ValidJson(request): ValidJson<BakerForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let dto: BakerForCreateDto = request.into();
    debug!("create_baker: {:?}", dto);
    let role_id = BakerMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/bakers/{baker_id}",
    tag = TAG,
    operation_id = "delete-baker-by-id",
    responses(
        (status = 200, description = "Baker is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState>,
    Path(baker_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    BakerMutation::delete(&state.conn, baker_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/bakers/{baker_id}",
    tag = TAG,
    operation_id = "get-baker-by-id",
    responses(
        (status = 200, description = "Baker Data", body = BakerDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState>,
    Path(baker_id): Path<i32>,
) -> Result<ResponseJson<BakerData>> {
    let baker = BakerQuery::get_by_id(&state.conn, baker_id).await?;
    Ok(ResponseJson(baker))
}

#[utoipa::path(
    get,
    path = "/bakers",
    tag = TAG,
    operation_id = "filter-baker",
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Filtered Baker", body = QueryResultResponse<BakerData>),
    )
)]
async fn filter(
    state: State<AppState>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: FilterParams<BakerDataFilterParams>,
) -> Result<ResponseJson<QueryResult<BakerData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();
    debug!("all_filters: {:?}", all_filters);

    let result = BakerQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/bakers", post(create))
        .route("/bakers/{baker_id}", delete(delete_by_id))
        .route("/bakers/{baker_id}", get(get_by_id))
        .route("/bakers", get(filter))
        .with_state(app_state.clone())
}
