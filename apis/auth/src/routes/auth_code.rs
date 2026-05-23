use axum::{
    extract::{Path, Query},
    routing::{delete, get, post},
    Router,
};
use features_auth_entities::auth_code::AuthCodeForCreateDto;
use tracing::debug;
use uuid::Uuid;

use features_auth_model::{
    auth_code::{
        AuthCodeData, AuthCodeDataFilterParams, AuthCodeDataResponse, AuthCodeForCreateRequest,
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
use features_auth_repo::auth_code::{AuthCodeMutation, AuthCodeQuery};

use crate::permission::{CanCreateAuthCode, CanDeleteAuthCode, CanReadAuthCode};

const TAG: &str = "auth_code";

#[utoipa::path(
    post,
    request_body = AuthCodeForCreateRequest,
    path = "/auth-codes",
    tag = TAG,
    summary = "Create auth code",
    responses(
        (status = 200, description = "Auth Code is created", body = OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn create_auth_code(
    _auth: Auth<CanCreateAuthCode>,
    ValidJson(register_request): ValidJson<AuthCodeForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: AuthCodeForCreateDto = register_request.into();
    let code_id = AuthCodeMutation::create(dto).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(code_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/auth-codes/{auth_code_id}",
    tag = TAG,
    summary = "Delete auth code",
    responses(
        (status = 200, description = "Auth Code is deleted", body = OkUuidResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Auth code not found", body = ErrorResponse),
    )
)]
async fn delete_auth_code(
    _auth: Auth<CanDeleteAuthCode>,
    Path(auth_code_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    AuthCodeMutation::delete(auth_code_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/auth-codes/{auth_code_id}",
    tag = TAG,
    summary = "Get auth code by ID",
    responses(
        (status = 200, description = "Auth Code Data", body = AuthCodeDataResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Auth code not found", body = ErrorResponse),
    )
)]
async fn get_auth_code(
    _auth: Auth<CanReadAuthCode>,
    Path(auth_code_id): Path<Uuid>,
) -> Result<ResponseJson<AuthCodeData>> {
    let auth_code = AuthCodeQuery::get(auth_code_id).await?;
    Ok(ResponseJson(auth_code))
}

#[utoipa::path(
    get,
    path = "/auth-codes",
    tag = TAG,
    summary = "Filter auth codes",
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Auth Code", body = QueryResultResponse<AuthCodeData>),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    )
)]
async fn filter_auth_codes(
    _auth: Auth<CanReadAuthCode>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<AuthCodeDataFilterParams>,
) -> Result<ResponseJson<QueryResult<AuthCodeData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = AuthCodeQuery::search(&pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/auth-codes", post(create_auth_code))
        .route("/auth-codes/{auth_code_id}", delete(delete_auth_code))
        .route("/auth-codes/{auth_code_id}", get(get_auth_code))
        .route("/auth-codes", get(filter_auth_codes))
        .with_state(app_state.clone())
}
