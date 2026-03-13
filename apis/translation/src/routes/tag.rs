use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_translation_model::{
    state::{TranslationAppState, TranslationCacheState},
    TagData, TagDataFilterParams, TagForCreateRequest, TagForUpdateRequest,
};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::ResponseJson,
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_translation_service::TagService;

const TAG: &str = "tag";

#[utoipa::path(
    post,
    path = "/tags",
    tag = TAG,
    request_body = TagForCreateRequest,
    responses(
        (status = 201, description = "Tag created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_tag(Json(req): Json<TagForCreateRequest>) -> Result<ResponseJson<OkUuid>> {
    let tag_id = TagService::create_tag(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(tag_id),
    }))
}

#[utoipa::path(
    get,
    path = "/tags/{tag_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Tag retrieved successfully", body = TagData),
    )
)]
async fn get_tag(Path(tag_id): Path<Uuid>) -> Result<ResponseJson<TagData>> {
    let tag = TagService::get_tag_by_id(tag_id).await?;
    Ok(ResponseJson(tag))
}

#[utoipa::path(
    get,
    path = "/tags",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered tags", body = QueryResultResponse<TagData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_tags(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TagDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TagData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = TagService::get_tags(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/tags/{tag_id}",
    tag = TAG,
    request_body = TagForUpdateRequest,
    responses(
        (status = 200, description = "Tag updated successfully", body = OkUuidResponse),
    )
)]
async fn update_tag(
    Path(tag_id): Path<Uuid>,
    Json(req): Json<TagForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TagService::update_tag(tag_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(tag_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/tags/{tag_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Tag deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_tag(Path(tag_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    TagService::delete_tag(tag_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(tag_id),
    }))
}

pub fn routes(app_state: &AppState<TranslationAppState, TranslationCacheState>) -> Router {
    Router::new()
        .route("/tags", post(create_tag))
        .route("/tags", get(filter_tags))
        .route("/tags/{tag_id}", get(get_tag))
        .route("/tags/{tag_id}", patch(update_tag))
        .route("/tags/{tag_id}", delete(delete_tag))
        .with_state(app_state.clone())
}
