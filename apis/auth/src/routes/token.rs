use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use shared_shared_auth::{
    claim::{AccessTokenStruct, AccessTokenStructResponse},
    data::{AuthorizationCodeData, AuthorizationCodeDataResponse},
};
use tracing::debug;
use uuid::Uuid;

use features_auth_model::{
    state::{AuthAppState, AuthCacheState},
    token::{
        TokenData, TokenDataFilterParams, TokenDataResponse, TokenForCreateRequest,
        TokenForVerifyRequest,
    },
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_auth_repo::token::TokenQuery;
use features_auth_service::TokenService;

const TAG: &str = "token";

#[utoipa::path(
    post,
    request_body = TokenForCreateRequest,
    path = "/public/tokens/oauth",
    tag = TAG,
    responses(
        (status = 200, description = "Token is created", body = AuthorizationCodeDataResponse),       
    )
)]
async fn create_token(
    state: State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<TokenForCreateRequest>,
) -> Result<ResponseJson<AuthorizationCodeData>> {
    debug!("Create token with request: {:?}", request);
    let cache = &state.cache;
    // Create Logic Service to convert request to DTO
    let authorization_code = TokenService::create_authorization_data(cache, &request).await?;
    let data = authorization_code.clone();

    Ok(ResponseJson(data))
}

#[utoipa::path(
    post,
    request_body = TokenForVerifyRequest,
    path = "/public/tokens/verify",
    tag = TAG,
    responses(
        (status = 200, description = "Token is verified", body = AccessTokenStructResponse),       
    )
)]
async fn verify_token(
    ValidJson(request): ValidJson<TokenForVerifyRequest>,
) -> Result<ResponseJson<AccessTokenStruct>> {
    debug!("Verify token with request: {:?}", request);
    // Create Logic Service to convert request to DTO
    let access_token_struct = TokenService::verify_token(&request).await?;

    Ok(ResponseJson(access_token_struct))
}

#[utoipa::path(
    get,
    path = "/tokens/{token_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Token Data", body = TokenDataResponse),       
    )
)]
async fn get_token(Path(token_id): Path<Uuid>) -> Result<ResponseJson<TokenData>> {
    let token = TokenQuery::get(token_id).await?;
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
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<TokenDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TokenData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = TokenQuery::search(&pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/public/tokens/oauth", post(create_token))
        .route("/public/tokens/verify", post(verify_token))
        .route("/tokens/{token_id}", get(get_token))
        .route("/tokens", get(filter_tokens))
        .with_state(app_state.clone())
}
