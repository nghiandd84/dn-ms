use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_tagging_model::{
    entity_tag::{
        BulkTagRequest, BulkTagResponse, BulkUntagRequest, EntityTagData,
        EntityTagForCreateRequest,
    },
    state::{TaggingAppState, TaggingCacheState},
};
use features_tagging_service::entity_tag::EntityTagService;

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::Auth;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::json::{ResponseJson, ValidJson};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
    query_params::QueryParams,
};
use shared_shared_extractor::TenantId;

use crate::permission::{CanCreateEntityTag, CanDeleteEntityTag, CanReadEntityTag};

const TAG: &str = "entity-tag";

#[utoipa::path(
    post,
    path = "/entity-tags",
    tag = TAG,
    request_body = EntityTagForCreateRequest,
    responses(
        (status = 201, description = "Tag assigned to entity", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn assign_tag(
    auth: Auth<CanCreateEntityTag>,
    TenantId(tenant_id): TenantId,
    ValidJson(req): ValidJson<EntityTagForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let user_id = auth.user_id;
    let id = EntityTagService::assign_tag(tenant_id, user_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    post,
    path = "/entity-tags/bulk",
    tag = TAG,
    request_body = BulkTagRequest,
    responses(
        (status = 200, description = "Tags bulk assigned", body = BulkTagResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn bulk_assign_tags(
    auth: Auth<CanCreateEntityTag>,
    TenantId(tenant_id): TenantId,
    ValidJson(req): ValidJson<BulkTagRequest>,
) -> Result<ResponseJson<BulkTagResponse>> {
    let user_id = auth.user_id;
    let result = EntityTagService::bulk_assign(tenant_id, user_id, req).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/entity-tags/bulk-remove",
    tag = TAG,
    request_body = BulkUntagRequest,
    responses(
        (status = 200, description = "Tags bulk removed", body = BulkTagResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn bulk_remove_tags(
    _auth: Auth<CanDeleteEntityTag>,
    ValidJson(req): ValidJson<BulkUntagRequest>,
) -> Result<ResponseJson<BulkTagResponse>> {
    let result = EntityTagService::bulk_remove(req).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/entities/{entity_type}/{entity_id}/tags",
    tag = TAG,
    params(Pagination),
    responses(
        (status = 200, description = "Tags for entity", body = QueryResultResponse<EntityTagData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_tags_for_entity(
    _auth: Auth<CanReadEntityTag>,
    TenantId(tenant_id): TenantId,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    Query(query_params): Query<QueryParams>,
) -> Result<ResponseJson<QueryResult<EntityTagData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result = EntityTagService::get_tags_for_entity(
        &tenant_id,
        &entity_type,
        entity_id,
        &pagination,
        &order,
        &query_params,
    )
    .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    get,
    path = "/tags/{tag_id}/entities",
    tag = TAG,
    params(Pagination),
    responses(
        (status = 200, description = "Entities for tag", body = QueryResultResponse<EntityTagData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_entities_for_tag(
    _auth: Auth<CanReadEntityTag>,
    TenantId(tenant_id): TenantId,
    Path(tag_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
) -> Result<ResponseJson<QueryResult<EntityTagData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let result =
        EntityTagService::get_entities_for_tag(&tenant_id, tag_id, &pagination, &order).await?;
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<TaggingAppState, TaggingCacheState>) -> Router {
    Router::new()
        .route("/entity-tags", post(assign_tag))
        .route("/entity-tags/bulk", post(bulk_assign_tags))
        .route("/entity-tags/bulk-remove", post(bulk_remove_tags))
        .route(
            "/entities/{entity_type}/{entity_id}/tags",
            get(get_tags_for_entity),
        )
        .route("/tags/{tag_id}/entities", get(get_entities_for_tag))
        .with_state(app_state.clone())
}
