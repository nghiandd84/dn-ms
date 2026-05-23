use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};

use tracing::debug;
use uuid::Uuid;

use features_auth_model::{
    client::{
        ClientData, ClientDataFilterParams, ClientDataResponse, ClientForCreateRequest,
        ClientForUpdateRequest,
    },
    state::AuthCacheState,
};

use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_model::state::AuthAppState;
use features_auth_repo::client::{ClientMutation, ClientQuery};

use crate::permission::{CanCreateClient, CanDeleteClient, CanReadClient, CanUpdateClient};

const TAG: &str = "client";

#[utoipa::path(
    post,
    request_body = ClientForCreateRequest,
    path = "/clients",
    tag = TAG,
    summary = "Create client",
    responses(
        (status = 200, description = "Client is created", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn create_client(
    _auth: Auth<CanCreateClient>,
    ValidJson(register_request): ValidJson<ClientForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let role_id = ClientMutation::create(register_request.into()).await?;
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
    summary = "Update client",
    description = "Change Client Data",
    responses(
        (status = 200, description= "Client was updated", body= OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Client not found", body = ErrorResponse),
    )
)]
async fn update_client(
    _auth: Auth<CanUpdateClient>,
    Path(client_id): Path<Uuid>,
    ValidJson(scope_request): ValidJson<ClientForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ClientMutation::update(client_id, scope_request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/clients/{client_id}",
    tag = TAG,
    summary = "Delete client",
    responses(
        (status = 200, description = "Client is deleted", body = OkUuidResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Client not found", body = ErrorResponse),
    )
)]
async fn delete_client(
    _auth: Auth<CanDeleteClient>,
    Path(client_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    ClientMutation::delete(client_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/clients/{client_id}",
    tag = TAG,
    summary = "Get client by ID",
    responses(
        (status = 200, description = "Client Data", body = ClientDataResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Client not found", body = ErrorResponse),
    )
)]
async fn get_client(
    _auth: Auth<CanReadClient>,
    Path(client_id): Path<Uuid>,
) -> Result<ResponseJson<ClientData>> {
    let scope = ClientQuery::get(client_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/clients",
    tag = TAG,
    summary = "Filter clients",
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Client", body = QueryResultResponse<ClientData>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn filter_clients(
    _auth: Auth<CanReadClient>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<ClientDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ClientData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = ClientQuery::search(&pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/clients", post(create_client))
        .route("/clients/{client_id}", patch(update_client))
        .route("/clients/{client_id}", delete(delete_client))
        .route("/clients/{client_id}", get(get_client))
        .route("/clients", get(filter_clients))
        .with_state(app_state.clone())
}
