use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Router,
};
use tracing::debug;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkI32, OkI32Response, OkUuid, Result},
};
use shared_shared_data_core::{
    order::Order,
    paging::{Pagination, QueryResult, QueryResultResponse},
};

use features_email_template_model::{
    state::EmailTemplateCacheState,
    template_translation::{
        TemplateTranslationData, TemplateTranslationDataFilterParams,
        TemplateTranslationDataResponse, TemplateTranslationForCreateRequest,
        TemplateTranslationForUpdateRequest,
    },
};
use features_email_template_service::TemplateTranslationService;

const TAG: &str = "Template-Translation";

#[utoipa::path(
    post,
    request_body = TemplateTranslationForCreateRequest,
    path = "/template-translations",
    tag = TAG,
    responses(
        (status = 200, description = "Template Translation was created", body = OkI32Response),
    )
)]
async fn create_template_translation(
    state: State<AppState<EmailTemplateCacheState>>,
    ValidJson(request): ValidJson<TemplateTranslationForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let translation_id = TemplateTranslationService::create(request).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(translation_id),
    }))
}

#[utoipa::path(
    patch,
    request_body = TemplateTranslationForUpdateRequest,
    params  (
        ("translation_id" = String, Path, description = "Template Translation Id"),
    ),
    path = "/template-translations/{translation_id}",
    tag = TAG,
    description = "Change Email Template",
    responses(
        (status = 200, description= "Template Translation was updated", body= OkI32Response),
    )
)]
async fn update_template_translation(
    state: State<AppState<EmailTemplateCacheState>>,
    Path(translation_id): Path<i32>,
    ValidJson(request): ValidJson<TemplateTranslationForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TemplateTranslationService::update(translation_id, request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/template-translations/{translation_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Template Translation was delete", body = OkI32Response),
    )
)]
async fn delete_template_translation(
    state: State<AppState<EmailTemplateCacheState>>,
    Path(translation_id): Path<i32>,
) -> Result<ResponseJson<OkUuid>> {
    TemplateTranslationService::delete(translation_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/template-translations/{translation_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Template Translation Data", body = TemplateTranslationDataResponse),
    )
)]
async fn get_template_translation(
    state: State<AppState<EmailTemplateCacheState>>,
    Path(translation_id): Path<i32>,
) -> Result<ResponseJson<TemplateTranslationData>> {
    let scope = TemplateTranslationService::get(&state.conn, translation_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/template-translations",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Template Translation", body = QueryResultResponse<TemplateTranslationData>),
    )
)]
async fn filter_template_translation(
    state: State<AppState<EmailTemplateCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<TemplateTranslationDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TemplateTranslationData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result =
        TemplateTranslationService::search(&state.conn, &pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<EmailTemplateCacheState>) -> Router {
    Router::new()
        .route("/template-translations", post(create_template_translation))
        .route(
            "/template-translations/{translation_id}",
            patch(update_template_translation),
        )
        .route(
            "/template-translations/{translation_id}",
            delete(delete_template_translation),
        )
        .route(
            "/template-translations/{translation_id}",
            get(get_template_translation),
        )
        .route("/template-translations", get(filter_template_translation))
        .with_state(app_state.clone())
}
