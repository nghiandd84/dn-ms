use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Router,
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_lookup_model::lookup_item_translation::{
    LookupItemTranslationData, LookupItemTranslationDataFilterParams,
    LookupItemTranslationForCreateRequest, LookupItemTranslationForUpdateRequest,
};
use features_lookup_model::state::{LookupAppState, LookupCacheState};
use features_lookup_service::lookup_item_translation::LookupItemTranslationService;

use shared_shared_app::state::AppState;
use shared_shared_data_app::result::{OkUuid, OkUuidResponse, Result};
use shared_shared_data_app::{
    filter_param::FilterParams,
    json::{ResponseJson, ValidJson},
};

const TAG: &str = "lookup-item-translation";

#[utoipa::path(
    get,
    path = "/lookup-types/{type_code}/items/{item_id}/translations",
    tag = TAG,
    responses(
        (status = 200, description = "List of translations", body = QueryResultResponse<LookupItemTranslationData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn get_translations(
    Path((_type_code, item_id)): Path<(String, Uuid)>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<LookupItemTranslationDataFilterParams>,
) -> Result<ResponseJson<QueryResult<LookupItemTranslationData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = LookupItemTranslationService::search_translations_by_item_id(
        item_id,
        &filters,
        &pagination,
        &order,
    )
    .await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    post,
    path = "/lookup-types/{type_code}/items/{item_id}/translations",
    tag = TAG,
    request_body = LookupItemTranslationForCreateRequest,
    responses(
        (status = 201, description = "Translation created", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn create_translation(
    Path((_type_code, item_id)): Path<(String, Uuid)>,
    ValidJson(mut req): ValidJson<LookupItemTranslationForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    req.lookup_item_id = Some(item_id);
    let id = LookupItemTranslationService::create_translation(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(id),
    }))
}

#[utoipa::path(
    patch,
    path = "/lookup-types/{type_code}/items/{item_id}/translations/{item_translation_id}",
    tag = TAG,
    request_body = LookupItemTranslationForUpdateRequest,
    responses(
        (status = 200, description = "Translation updated", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn update_translation(
    Path((_type_code, item_id, item_translation_id)): Path<(String, Uuid, Uuid)>,
    ValidJson(req): ValidJson<LookupItemTranslationForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    LookupItemTranslationService::update_translation(item_translation_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(item_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/lookup-types/{type_code}/items/{item_id}/translations/{item_translation_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Translation deleted", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
pub async fn delete_translation(
    Path((_type_code, _item_id, item_translation_id)): Path<(String, Uuid, Uuid)>,
) -> Result<ResponseJson<OkUuid>> {
    LookupItemTranslationService::delete_translation(item_translation_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(item_translation_id),
    }))
}

pub fn routes(app_state: &AppState<LookupAppState, LookupCacheState>) -> Router {
    Router::new()
        .route(
            "/lookup-types/{type_code}/items/{item_id}/translations",
            get(get_translations),
        )
        .route(
            "/lookup-types/{type_code}/items/{item_id}/translations",
            post(create_translation),
        )
        .route(
            "/lookup-types/{type_code}/items/{item_id}/translations/{item_translation_id}",
            patch(update_translation),
        )
        .route(
            "/lookup-types/{type_code}/items/{item_id}/translations/{item_translation_id}",
            delete(delete_translation),
        )
        .with_state(app_state.clone())
}
