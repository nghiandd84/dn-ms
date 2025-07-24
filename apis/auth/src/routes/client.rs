use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};

use tracing::debug;
use uuid::Uuid;

use features_auth_model::client::{
    ClientData, ClientDataFilterParams, ClientDataResponse, ClientForCreateRequest,
    ClientForUpdateRequest,
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

use features_auth_service::client::{ClientMutation, ClientQuery};
use features_auth_model::state::AuthCacheState;

const TAG: &str = "client";

#[utoipa::path(
    post,
    request_body = ClientForCreateRequest,
    path = "/clients",
    tag = TAG,
    responses(
        (status = 200, description = "Client is created", body = OkUuidResponse),       
    )
)]
async fn create_client(
    state: State<AppState<AuthCacheState>>,
    ValidJson(register_request): ValidJson<ClientForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let role_id = ClientMutation::create(&state.conn, register_request.into()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(role_id),
    }))
}

#[utoipa::path(
    patch,
    request_body = ClientForUpdateRequest,
    params  (
        ("client_id" = String, Path, description = "Client Id"),
    ),
    path = "/clients/{client_id}",
    tag = TAG,
    description = "Change Client Data",
    responses(
        (status = 200, description= "Client was updated", body= OkUuidResponse),       
    )
)]
async fn update_client(
    state: State<AppState<AuthCacheState>>,
    Path(client_id): Path<Uuid>,
    ValidJson(scope_request): ValidJson<ClientForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ClientMutation::update(&state.conn, client_id, scope_request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/clients/{client_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Client is deleted", body = OkUuidResponse),       
    )
)]
async fn delete_client(
    state: State<AppState<AuthCacheState>>,
    Path(client_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    ClientMutation::delete(&state.conn, client_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/clients/{client_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Client Data", body = ClientDataResponse),       
    )
)]
async fn get_client(
    state: State<AppState<AuthCacheState>>,
    Path(client_id): Path<Uuid>,
) -> Result<ResponseJson<ClientData>> {
    let scope = ClientQuery::get(&state.conn, client_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/clients",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Client", body = QueryResultResponse<ClientData>),       
    )
)]
async fn filter_clients(
    state: State<AppState<AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<ClientDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ClientData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = ClientQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/clients", post(create_client))
        .route("/clients/{client_id}", patch(update_client))
        .route("/clients/{client_id}", delete(delete_client))
        .route("/clients/{client_id}", get(get_client))
        .route("/clients", get(filter_clients))
        .with_state(app_state.clone())
}
