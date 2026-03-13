use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_translation_model::{
    state::{TranslationAppState, TranslationCacheState},
    AssignTagsRequest, TranslationKeyData, TranslationKeyDataFilterParams,
    TranslationKeyForCreateRequest, TranslationKeyForUpdateRequest, UnassignTagsRequest,
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

use features_translation_service::TranslationKeyService;

const TAG: &str = "translation_key";

#[utoipa::path(
    post,
    path = "/translation-keys",
    tag = TAG,
    request_body = TranslationKeyForCreateRequest,
    responses(
        (status = 201, description = "Translation key created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_translation_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Json(req): Json<TranslationKeyForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let key_id = TranslationKeyService::create_translation_key(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

#[utoipa::path(
    get,
    path = "/translation-keys/{key_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Translation key retrieved successfully", body = TranslationKeyData),
    )
)]
async fn get_translation_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(key_id): Path<Uuid>,
) -> Result<ResponseJson<TranslationKeyData>> {
    let key = TranslationKeyService::get_translation_key_by_id(&state.conn, key_id).await?;
    Ok(ResponseJson(key))
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}/translation-keys",
    tag = TAG,
    responses(
        (status = 200, description = "Translation keys for project retrieved successfully", body = Vec<TranslationKeyData>),
    )
)]
async fn get_translation_keys_by_project(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(project_id): Path<Uuid>,
) -> Result<ResponseJson<QueryResult<TranslationKeyData>>> {
    let keys =
        TranslationKeyService::get_translation_keys_by_project(&state.conn, project_id).await?;
    Ok(ResponseJson(keys))
}

#[utoipa::path(
    get,
    path = "/translation-keys",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered translation keys", body = QueryResultResponse<TranslationKeyData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_translation_keys(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<TranslationKeyDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TranslationKeyData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result =
        TranslationKeyService::get_translation_keys(&state.conn, &pagination, &order, &filters)
            .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/translation-keys/{key_id}",
    tag = TAG,
    request_body = TranslationKeyForUpdateRequest,
    responses(
        (status = 200, description = "Translation key updated successfully", body = OkUuidResponse),
    )
)]
async fn update_translation_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(key_id): Path<Uuid>,
    Json(req): Json<TranslationKeyForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TranslationKeyService::update_translation_key(key_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/translation-keys/{key_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Translation key deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_translation_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(key_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    TranslationKeyService::delete_translation_key(key_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

#[utoipa::path(
    post,
    path = "/translation-keys/{key_id}/assign-tags",
    tag = TAG,
    request_body = AssignTagsRequest,
    responses(
        (status = 200, description = "Tags assigned to translation key successfully", body = OkUuidResponse),
    )
)]
async fn assign_tags_to_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(key_id): Path<Uuid>,
    Json(req): Json<AssignTagsRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TranslationKeyService::assign_tags_to_key(&state.conn, key_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

#[utoipa::path(
    post,
    path = "/translation-keys/{key_id}/unassign-tags",
    tag = TAG,
    request_body = UnassignTagsRequest,
    responses(
        (status = 200, description = "Tags unassigned from translation key successfully", body = OkUuidResponse),
    )
)]
async fn unassign_tags_from_key(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(key_id): Path<Uuid>,
    Json(req): Json<UnassignTagsRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TranslationKeyService::unassign_tags_from_key(key_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(key_id),
    }))
}

pub fn routes(app_state: &AppState<TranslationAppState, TranslationCacheState>) -> Router {
    Router::new()
        .route("/translation-keys", post(create_translation_key))
        .route("/translation-keys", get(filter_translation_keys))
        .route("/translation-keys/{key_id}", get(get_translation_key))
        .route("/translation-keys/{key_id}", patch(update_translation_key))
        .route("/translation-keys/{key_id}", delete(delete_translation_key))
        .route(
            "/translation-keys/{key_id}/assign-tags",
            post(assign_tags_to_key),
        )
        .route(
            "/translation-keys/{key_id}/unassign-tags",
            post(unassign_tags_from_key),
        )
        .route(
            "/projects/{project_id}/translation-keys",
            get(get_translation_keys_by_project),
        )
        .with_state(app_state.clone())
}
