use axum::{
    extract::{Path, Query},
    routing::{delete, get, post},
    Router,
};
use tracing::{debug, instrument, Level};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::json::{ResponseJson, ValidJson};
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_url_shortener_model::{
    api_key::{ApiKeyCreatedResponse, ApiKeyData, CreateApiKeyRequest},
    state::{UrlShortenerAppState, UrlShortenerCacheState},
};
use features_url_shortener_service::api_key::ApiKeyService;

use crate::permission::{CanDeleteApiKey, CanManageApiKeys};

const TAG: &str = "api-key";

#[utoipa::path(
    post,
    path = "/api-keys",
    tag = TAG,
    request_body = CreateApiKeyRequest,
    responses(
        (status = 201, description = "API key created", body = ApiKeyCreatedResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_api_key(
    auth: Auth<CanManageApiKeys>,
    ValidJson(req): ValidJson<CreateApiKeyRequest>,
) -> Result<ResponseJson<ApiKeyCreatedResponse>> {
    let user_id = auth.user_id;
    debug!("Creating API key for user: {}", user_id);
    let result = ApiKeyService::create_api_key(user_id, req).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/api-keys",
    tag = TAG,
    params(Pagination, Order),
    responses(
        (status = 200, description = "List of API keys", body = QueryResultResponse<ApiKeyData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_api_keys(
    auth: Auth<CanManageApiKeys>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<ApiKeyData>>> {
    let user_id = auth.user_id;
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = ApiKeyService::list_user_keys(&user_id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    delete,
    path = "/api-keys/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "API key deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_api_key(
    auth: Auth<CanDeleteApiKey>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    let user_id = auth.user_id;
    ApiKeyService::revoke_api_key(id, user_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<UrlShortenerAppState, UrlShortenerCacheState>) -> Router {
    Router::new()
        .route("/api-keys", post(create_api_key))
        .route("/api-keys", get(get_api_keys))
        .route("/api-keys/{id}", delete(delete_api_key))
        .with_state(app_state.clone())
}
