use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::instrument;

use features_merchant_model::api_key::{
    ApiKeyData, ApiKeyDataFilterParams, ApiKeyForCreateRequest, ApiKeyForUpdateRequest,
};
use features_merchant_model::state::{MerchantAppState, MerchantCacheState};
use features_merchant_service::ApiKeyService;

use shared_shared_auth::permission::Auth;

use crate::permission::{CanCreateApiKey, CanDeleteApiKey, CanReadApiKey, CanUpdateApiKey};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
    result::{OkI32, OkI32Response, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

const TAG: &str = "api_key";

#[utoipa::path(
    post,
    path = "/api-keys",
    tag = TAG,
    request_body = ApiKeyForCreateRequest,
    responses(
        (status = 200, description = "API key created", body = OkI32Response),
    )
)]
#[instrument(skip_all)]
async fn create_api_key(
    _auth: Auth<CanCreateApiKey>,
    ValidJson(req): ValidJson<ApiKeyForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let id = ApiKeyService::create_api_key(req).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    get,
    path = "/api-keys/{api_key_id}",
    tag = TAG,
    responses(
        (status = 200, description = "API key retrieved", body = ApiKeyData),
    )
)]
async fn get_api_key(_auth: Auth<CanReadApiKey>, Path(api_key_id): Path<i32>) -> Result<ResponseJson<ApiKeyData>> {
    let item = ApiKeyService::get_api_key_by_id(api_key_id).await?;
    Ok(ResponseJson(item))
}

#[utoipa::path(
    get,
    path = "/api-keys",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered API keys", body = QueryResultResponse<ApiKeyData>),
    )
)]
#[instrument(skip_all)]
async fn filter_api_keys(
    _auth: Auth<CanReadApiKey>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<ApiKeyDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ApiKeyData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = ApiKeyService::get_api_keys(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/api-keys/{api_key_id}",
    tag = TAG,
    request_body = ApiKeyForUpdateRequest,
    responses(
        (status = 200, description = "API key updated", body = OkI32Response),
    )
)]
#[instrument(skip_all)]
async fn update_api_key(
    _auth: Auth<CanUpdateApiKey>,
    Path(api_key_id): Path<i32>,
    ValidJson(req): ValidJson<ApiKeyForUpdateRequest>,
) -> Result<ResponseJson<OkI32>> {
    ApiKeyService::update_api_key(api_key_id, req).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(api_key_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/api-keys/{api_key_id}",
    tag = TAG,
    responses(
        (status = 200, description = "API key deleted", body = OkI32Response),
    )
)]
#[instrument(skip_all)]
async fn delete_api_key(_auth: Auth<CanDeleteApiKey>, Path(api_key_id): Path<i32>) -> Result<ResponseJson<OkI32>> {
    ApiKeyService::delete_api_key(api_key_id).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(api_key_id),
    }))
}

pub fn routes(app_state: &AppState<MerchantAppState, MerchantCacheState>) -> Router {
    Router::new()
        .route("/api-keys", post(create_api_key))
        .route("/api-keys", get(filter_api_keys))
        .route("/api-keys/{api_key_id}", get(get_api_key))
        .route("/api-keys/{api_key_id}", patch(update_api_key))
        .route("/api-keys/{api_key_id}", delete(delete_api_key))
        .with_state(app_state.clone())
}
