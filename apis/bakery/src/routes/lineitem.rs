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


use features_bakery_model::{lineitem::{
    LineitemData, LineitemDataFilterParams, LineitemDataResponse, LineitemForCreateRequest,
}, state::BakeryCacheState};
use features_bakery_service::lineitem::{LineitemMutation, LineitemQuery};

const TAG: &str = "lineitem";

#[utoipa::path(
    post,
    request_body = LineitemForCreateRequest,
    path = "/lineitems",
    operation_id = "create-lineitem",
    tag = TAG,
    responses(
        (status = 200, description = "Lineitem is created", body = OkI32Response),       
    )
)]
async fn create(
    state: State<AppState<BakeryCacheState>>,
    ValidJson(request): ValidJson<LineitemForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let role_id = LineitemMutation::create(&state.conn, request.into()).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/lineitems/{lineitem_id}",
    tag = TAG,
    operation_id = "delete-lineitem-by-id",
    responses(
        (status = 200, description = "Lineitem is deleted", body = OkI32Response),
    )
)]
async fn delete_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(lineitem_id): Path<i32>,
) -> Result<ResponseJson<OkI32>> {
    LineitemMutation::delete(&state.conn, lineitem_id).await?;
    Ok(ResponseJson(OkI32 { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/lineitems/{lineitem_id}",
    operation_id = "get-lineitem-by-id",
    tag = TAG,
    responses(
        (status = 200, description = "Lineitem Data", body = LineitemDataResponse),
    )
)]
async fn get_by_id(
    state: State<AppState<BakeryCacheState>>,
    Path(lineitem_id): Path<i32>,
) -> Result<ResponseJson<LineitemData>> {
    let cake = LineitemQuery::get_by_id(&state.conn, lineitem_id).await?;
    Ok(ResponseJson(cake))
}

#[utoipa::path(
    get,
    path = "/lineitems",
    operation_id = "filter-lineitem",
    tag = TAG,
    params  (
       Order,
       Pagination
    ),
    responses(
        (status = 200, description = "Lineitem Customer", body = QueryResultResponse<LineitemData>),
    )
)]
async fn filter(
    state: State<AppState<BakeryCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<LineitemDataFilterParams>,
) -> Result<ResponseJson<QueryResult<LineitemData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = LineitemQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState<BakeryCacheState>) -> Router {
    Router::new()
        .route("/lineitems", post(create))
        .route("/lineitems/{lineitem_id}", delete(delete_by_id))
        .route("/lineitems/{lineitem_id}", get(get_by_id))
        .route("/lineitems", get(filter))
        .with_state(app_state.clone())
}
