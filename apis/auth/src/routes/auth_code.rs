use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Router,
};
use features_auth_entities::auth_code::AuthCodeForCreateDto;
use tracing::debug;
use uuid::Uuid;

use features_auth_model::auth_code::{
    AuthCodeData, AuthCodeDataFilterParams, AuthCodeDataResponse, AuthCodeForCreateRequest
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_service::auth_code::{AuthCodeMutation, AuthCodeQuery};
use features_auth_model::state::AuthCacheState;

const TAG: &str = "auth_code";

#[utoipa::path(
    post,
    request_body = AuthCodeForCreateRequest,
    path = "/auth-codes",
    tag = TAG,
    responses(
        (status = 200, description = "Auth Code is created", body = OkUuidResponse),       
    )
)]
async fn create_auth_code(
    state: State<AppState<AuthCacheState>>,
    ValidJson(register_request): ValidJson<AuthCodeForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: AuthCodeForCreateDto = register_request.into();
    let role_id = AuthCodeMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(role_id),
    }))
}


#[utoipa::path(
    delete,
    path = "/auth-codes/{auth_code_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Auth Code is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_auth_code(
    state: State<AppState<AuthCacheState>>,
    Path(auth_code_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    AuthCodeMutation::delete(&state.conn, auth_code_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/auth-codes/{auth_code_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Auth Code Data", body = AuthCodeDataResponse),       
    )
)]
async fn get_auth_code(
    state: State<AppState<AuthCacheState>>,
    Path(auth_code_id): Path<Uuid>,
) -> Result<ResponseJson<AuthCodeData>> {
    let auth_code = AuthCodeQuery::get(&state.conn, auth_code_id).await?;
    Ok(ResponseJson(auth_code))
}

#[utoipa::path(
    get,
    path = "/auth-codes",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Auth Code", body = QueryResultResponse<AuthCodeData>),       
    )
)]
async fn filter_auth_codes(
    state: State<AppState<AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<AuthCodeDataFilterParams>,
) -> Result<ResponseJson<QueryResult<AuthCodeData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = AuthCodeQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/auth-codes", post(create_auth_code))
        .route("/auth-codes/{auth_code_id}", delete(delete_auth_code))
        .route("/auth-codes/{auth_code_id}", get(get_auth_code))
        .route("/auth-codes", get(filter_auth_codes))
        .with_state(app_state.clone())
}
