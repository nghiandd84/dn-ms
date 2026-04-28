use axum::{
    extract::{Path, Query},
    routing::{delete, get},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_auth_model::{
    state::{AuthAppState, AuthCacheState},
    user::{UserData, UserDataFilterParams, UserDataResponse},
};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_repo::user::{UserMutation, UserQuery};

use crate::permission::{CanDeleteUser, CanReadUser};

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
    _auth: Auth<CanDeleteUser>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    UserMutation::delete_user(user_id).await?;
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
    _auth: Auth<CanReadUser>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<UserData>> {
    let user_dto = UserQuery::get(user_id).await?;
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
#[instrument(level = Level::INFO, skip_all)]
async fn filter_users(
    _auth: Auth<CanReadUser>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<UserDataFilterParams>,
) -> Result<ResponseJson<QueryResult<UserData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = UserQuery::search(&pagination, &order, &all_filters).await?;
    Ok(ResponseJson(result))
}


pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/users/{user_id}", delete(delete_user))
        .route("/users/{user_id}", get(get_user))
        .route("/users", get(filter_users))
        .with_state(app_state.clone())
}
