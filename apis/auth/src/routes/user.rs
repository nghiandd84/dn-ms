use axum::{
    extract::{Path, Query, State},
    routing::{delete, get},
    Router,
};
use uuid::Uuid;

use features_auth_model::{
    state::AuthCacheState,
    user::{UserData, UserDataFilterParams, UserDataResponse},
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_service::user::{UserMutation, UserQuery};

const TAG: &str = "user";

#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    tag = TAG,
    responses(
        (status = 200, description = "User is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_user(
    state: State<AppState<AuthCacheState>>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    UserMutation::delete_user(&state.conn, user_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = TAG,
    responses(
        (status = 200, description = "User is deleted", body = UserDataResponse),       
    )
)]
async fn get_user(
    state: State<AppState<AuthCacheState>>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<UserData>> {
    let user_dto = UserQuery::get(&state.conn, user_id).await?;
    Ok(ResponseJson(user_dto))
}

#[utoipa::path(
    get,
    path = "/users",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered user data", body = QueryResultResponse<UserData>),       
    )
)]
async fn filter_users(
    state: State<AppState<AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<UserDataFilterParams>,
) -> Result<ResponseJson<QueryResult<UserData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = UserQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    Ok(ResponseJson(result))
}

async fn test_filters(
    state: State<AppState<AuthCacheState>>,
) -> Result<ResponseJson<QueryResult<UserData>>> {
    let pagination = Pagination::default();
    let order = Order::default();
    let all_filters = vec![];

    let result = UserQuery::advance_search(&state.conn, &pagination, &order, &all_filters).await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/users/{user_id}", delete(delete_user))
        .route("/users/{user_id}", get(get_user))
        .route("/users", get(filter_users))
        .route("/test_users", get(test_filters))
        .with_state(app_state.clone())
}
