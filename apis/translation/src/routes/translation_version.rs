use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_translation_model::{
    state::{TranslationAppState, TranslationCacheState},
    TranslationVersionData, TranslationVersionDataFilterParams, TranslationVersionForCreateRequest,
    TranslationVersionForUpdateRequest,
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

use features_translation_service::TranslationVersionService;

const TAG: &str = "translation_version";

#[utoipa::path(
    post,
    path = "/translation-versions",
    tag = TAG,
    request_body = TranslationVersionForCreateRequest,
    responses(
        (status = 201, description = "Translation version created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_translation_version(
    Json(req): Json<TranslationVersionForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let version_id = TranslationVersionService::create_translation_version(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(version_id),
    }))
}

#[utoipa::path(
    get,
    path = "/translation-versions/{version_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Translation version retrieved successfully", body = TranslationVersionData),
    )
)]
async fn get_translation_version(
    Path(version_id): Path<Uuid>,
) -> Result<ResponseJson<TranslationVersionData>> {
    let version = TranslationVersionService::get_translation_version_by_id(version_id).await?;
    Ok(ResponseJson(version))
}

#[utoipa::path(
    get,
    path = "/translation-keys/{key_id}/versions/latest",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Latest translation version retrieved successfully", body = Option<TranslationVersionData>),
    )
)]
async fn get_latest_translation_version(
    Path(key_id): Path<Uuid>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TranslationVersionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TranslationVersionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let version = TranslationVersionService::get_latest_version_by_key_locale(
        key_id,
        &filters,
        &pagination,
        &order,
    )
    .await?;
    Ok(ResponseJson(version))
}

#[utoipa::path(
    get,
    path = "/translation-versions",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered translation versions", body = QueryResultResponse<TranslationVersionData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_translation_versions(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TranslationVersionDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TranslationVersionData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        TranslationVersionService::get_translation_versions(&pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/translation-versions/{version_id}",
    tag = TAG,
    request_body = TranslationVersionForUpdateRequest,
    responses(
        (status = 200, description = "Translation version updated successfully", body = OkUuidResponse),
    )
)]
async fn update_translation_version(
    Path(version_id): Path<Uuid>,
    Json(req): Json<TranslationVersionForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TranslationVersionService::update_translation_version(version_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(version_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/translation-versions/{version_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Translation version deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_translation_version(Path(version_id): Path<Uuid>) -> Result<ResponseJson<OkUuid>> {
    TranslationVersionService::delete_translation_version(version_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(version_id),
    }))
}

pub fn routes(app_state: &AppState<TranslationAppState, TranslationCacheState>) -> Router {
    Router::new()
        .route("/translation-versions", post(create_translation_version))
        .route("/translation-versions", get(filter_translation_versions))
        .route(
            "/translation-versions/{version_id}",
            get(get_translation_version),
        )
        .route(
            "/translation-versions/{version_id}",
            patch(update_translation_version),
        )
        .route(
            "/translation-versions/{version_id}",
            delete(delete_translation_version),
        )
        .route(
            "/translation-keys/{key_id}/versions/latest",
            get(get_latest_translation_version),
        )
        .with_state(app_state.clone())
}
