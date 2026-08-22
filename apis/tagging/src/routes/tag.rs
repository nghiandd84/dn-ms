use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use serde::Deserialize;
use tracing::{instrument, Level};
use uuid::Uuid;

use features_tagging_model::{
    state::{TaggingAppState, TaggingCacheState},
    tag::{
        TagData, TagDataFilterParams, TagForCreateRequest, TagForUpdateRequest, TagMergeRequest,
        TagUsageResponse,
    },
};
use features_tagging_service::tag::TagService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
    query_params::QueryParams,
};
use shared_shared_extractor::TenantId;

use crate::permission::{CanCreateTag, CanDeleteTag, CanReadTag, CanUpdateTag};

const TAG: &str = "tag";

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub search: Option<String>,
    pub limit: Option<u64>,
}

#[utoipa::path(
    get,
    path = "/tags",
    tag = TAG,
    params(Pagination),
    responses(
        (status = 200, description = "List tags", body = QueryResultResponse<TagData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tags(
    _auth: Auth<CanReadTag>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TagDataFilterParams>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<TagData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = TagService::get_tags(&filters, &pagination, &order, &query_params).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/tags/search",
    tag = TAG,
    responses(
        (status = 200, description = "Search tags (autocomplete)", body = Vec<TagData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn search_tags(
    _auth: Auth<CanReadTag>,
    TenantId(tenant_id): TenantId,
    Query(params): Query<SearchParams>,
) -> Result<ResponseJson<Vec<TagData>>> {
    let search_term = params.search.unwrap_or_default();
    if search_term.is_empty() {
        return Ok(ResponseJson(vec![]));
    }
    let results = TagService::search_tags(&tenant_id, &search_term, params.limit).await?;
    Ok(ResponseJson(results))
}

#[utoipa::path(
    get,
    path = "/tags/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Get tag by ID", body = TagData),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tag(
    _auth: Auth<CanReadTag>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<TagData>> {
    let result = TagService::get_tag_by_id(id).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/tags",
    tag = TAG,
    request_body = TagForCreateRequest,
    responses(
        (status = 201, description = "Tag created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_tag(
    _auth: Auth<CanCreateTag>,
    TenantId(tenant_id): TenantId,
    ValidJson(req): ValidJson<TagForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = TagService::create_tag(tenant_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    patch,
    path = "/tags/{id}",
    tag = TAG,
    request_body = TagForUpdateRequest,
    responses(
        (status = 200, description = "Tag updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_tag(
    _auth: Auth<CanUpdateTag>,
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<TagForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TagService::update_tag(id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/tags/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Tag deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_tag(
    _auth: Auth<CanDeleteTag>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    TagService::delete_tag(id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    get,
    path = "/tags/{id}/usage",
    tag = TAG,
    responses(
        (status = 200, description = "Tag usage count", body = TagUsageResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tag_usage(
    _auth: Auth<CanReadTag>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<TagUsageResponse>> {
    let result = TagService::get_usage_count(id).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/tags/{id}/merge",
    tag = TAG,
    request_body = TagMergeRequest,
    responses(
        (status = 200, description = "Tag merged", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn merge_tag(
    _auth: Auth<CanUpdateTag>,
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<TagMergeRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TagService::merge_tags(id, req.target_tag_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(req.target_tag_id),
    }))
}

pub fn routes(app_state: &AppState<TaggingAppState, TaggingCacheState>) -> Router {
    Router::new()
        .route("/tags", get(get_tags))
        .route("/tags/search", get(search_tags))
        .route("/tags", post(create_tag))
        .route("/tags/{id}", get(get_tag))
        .route("/tags/{id}", patch(update_tag))
        .route("/tags/{id}", delete(delete_tag))
        .route("/tags/{id}/usage", get(get_tag_usage))
        .route("/tags/{id}/merge", post(merge_tag))
        .with_state(app_state.clone())
}
