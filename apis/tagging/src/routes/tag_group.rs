use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_tagging_model::{
    state::{TaggingAppState, TaggingCacheState},
    tag_group::{TagGroupData, TagGroupDataFilterParams, TagGroupForCreateRequest, TagGroupForUpdateRequest},
};
use features_tagging_service::tag_group::TagGroupService;

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

use crate::permission::{
    CanCreateTagGroup, CanDeleteTagGroup, CanReadTagGroup, CanUpdateTagGroup,
};

const TAG: &str = "tag-group";

#[utoipa::path(
    get,
    path = "/tag-groups",
    tag = TAG,
    params(Pagination),
    responses(
        (status = 200, description = "List tag groups", body = QueryResultResponse<TagGroupData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tag_groups(
    _auth: Auth<CanReadTagGroup>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TagGroupDataFilterParams>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<TagGroupData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        TagGroupService::get_tag_groups(&filters, &pagination, &order, &query_params).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/tag-groups/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Get tag group by ID", body = TagGroupData),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tag_group(
    _auth: Auth<CanReadTagGroup>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<TagGroupData>> {
    let result = TagGroupService::get_tag_group_by_id(id).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/tag-groups",
    tag = TAG,
    request_body = TagGroupForCreateRequest,
    responses(
        (status = 201, description = "Tag group created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_tag_group(
    _auth: Auth<CanCreateTagGroup>,
    TenantId(tenant_id): TenantId,
    ValidJson(req): ValidJson<TagGroupForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let id = TagGroupService::create_tag_group(tenant_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    patch,
    path = "/tag-groups/{id}",
    tag = TAG,
    request_body = TagGroupForUpdateRequest,
    responses(
        (status = 200, description = "Tag group updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_tag_group(
    _auth: Auth<CanUpdateTagGroup>,
    Path(id): Path<Uuid>,
    ValidJson(req): ValidJson<TagGroupForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TagGroupService::update_tag_group(id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    delete,
    path = "/tag-groups/{id}",
    tag = TAG,
    responses(
        (status = 200, description = "Tag group deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_tag_group(
    _auth: Auth<CanDeleteTagGroup>,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    TagGroupService::delete_tag_group(id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

pub fn routes(app_state: &AppState<TaggingAppState, TaggingCacheState>) -> Router {
    Router::new()
        .route("/tag-groups", get(get_tag_groups))
        .route("/tag-groups", post(create_tag_group))
        .route("/tag-groups/{id}", get(get_tag_group))
        .route("/tag-groups/{id}", patch(update_tag_group))
        .route("/tag-groups/{id}", delete(delete_tag_group))
        .with_state(app_state.clone())
}
