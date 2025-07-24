use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};
use features_auth_entities::scope::ScopeForCreateDto;
use tracing::debug;
use uuid::Uuid;

use features_auth_model::{scope::{
    ScopeData, ScopeDataFilterParams, ScopeDataResponse, ScopeForCreateRequest,
    ScopeForUpdateRequest,
}, state::AuthCacheState};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_service::scope::{ScopeMutation, ScopeQuery};


const TAG: &str = "scope";

#[utoipa::path(
    post,
    request_body = ScopeForCreateRequest,
    path = "/scopes",
    tag = TAG,
    responses(
        (status = 200, description = "Scope is created", body = OkUuidResponse),       
    )
)]
async fn create_scope(
    state: State<AppState<AuthCacheState>>,
    ValidJson(register_request): ValidJson<ScopeForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: ScopeForCreateDto = register_request.into();
    let role_id = ScopeMutation::create(&state.conn, dto).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    patch,
    request_body = ScopeForUpdateRequest,
    params  (
        ("scope_id" = String, Path, description = "Scope Id"),
    ),
    path = "/scopes/{scope_id}",
    tag = TAG,
    description = "Change Scope Data",
    responses(
        (status = 200, description= "Scope was updated", body= OkUuidResponse),       
    )
)]
async fn update_scope(
    state: State<AppState<AuthCacheState>>,
    Path(scope_id): Path<Uuid>,
    ValidJson(scope_request): ValidJson<ScopeForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ScopeMutation::update(&state.conn, scope_id, scope_request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/scopes/{scope_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Scope is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_scope(
    state: State<AppState<AuthCacheState>>,
    Path(scope_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    ScopeMutation::delete(&state.conn, scope_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/scopes/{scope_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Scope Data", body = ScopeDataResponse),       
    )
)]
async fn get_scope(
    state: State<AppState<AuthCacheState>>,
    Path(scope_id): Path<Uuid>,
) -> Result<ResponseJson<ScopeData>> {
    let scope = ScopeQuery::get(&state.conn, scope_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/scopes",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Scope", body = QueryResultResponse<ScopeData>),       
    )
)]
async fn filter_scopes(
    state: State<AppState<AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<ScopeDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ScopeData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = ScopeQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/scopes", post(create_scope))
        .route("/scopes/{scope_id}", patch(update_scope))
        .route("/scopes/{scope_id}", delete(delete_scope))
        .route("/scopes/{scope_id}", get(get_scope))
        .route("/scopes", get(filter_scopes))
        .with_state(app_state.clone())
}
