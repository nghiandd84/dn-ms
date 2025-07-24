use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use shared_shared_auth::data::AuthorizationCodeData;
use tracing::debug;
use uuid::Uuid;

use features_auth_model::{state::AuthCacheState, token::{
    TokenData, TokenDataFilterParams, TokenDataResponse, TokenForCreateRequest,
}};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_service::{services::TokenService, token::TokenQuery};


const TAG: &str = "token";

#[utoipa::path(
    post,
    request_body = TokenForCreateRequest,
    path = "/oauth/token",
    tag = TAG,
    responses(
        (status = 200, description = "Token is created", body = OkUuidResponse),       
    )
)]
async fn create_token(
    state: State<AppState<AuthCacheState>>,
    ValidJson(request): ValidJson<TokenForCreateRequest>,
) -> Result<ResponseJson<AuthorizationCodeData>> {
    debug!("Create token with request: {:?}", request);
    // Create Logic Service to convert request to DTO
    let authorization_code = TokenService::create_authorization_data(&state.conn, &request).await?;
    let data = authorization_code.clone();

    Ok(ResponseJson(data))
}

#[utoipa::path(
    get,
    path = "/tokens/{token_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Token Data", body = TokenDataResponse),       
    )
)]
async fn get_token(
    state: State<AppState<AuthCacheState>>,
    Path(token_id): Path<Uuid>,
) -> Result<ResponseJson<TokenData>> {
    let token = TokenQuery::get(&state.conn, token_id).await?;
    Ok(ResponseJson(token))
}

#[utoipa::path(
    get,
    path = "/tokens",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Token", body = QueryResultResponse<TokenData>),       
    )
)]
async fn filter_tokens(
    state: State<AppState<AuthCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<TokenDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TokenData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = TokenQuery::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/oauth/token", post(create_token))
        .route("/tokens/{token_id}", get(get_token))
        .route("/tokens", get(filter_tokens))
        .with_state(app_state.clone())
}
