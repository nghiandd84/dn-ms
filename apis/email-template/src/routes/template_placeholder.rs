use axum::{
    extract::{Path, Query},
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
    template_placeholder::{
        TemplatePlaceholderData, TemplatePlaceholderDataFilterParams,
        TemplatePlaceholderDataResponse, TemplatePlaceholderForCreateRequest,
        TemplatePlaeholderForUpdateRequest,
    },
};
use features_email_template_service::TemplatePlaceholderService;

const TAG: &str = "Template-Plaeceholder";

#[utoipa::path(
    post,
    request_body = TemplatePlaceholderForCreateRequest,
    path = "/template-placeholders",
    tag = TAG,
    responses(
        (status = 200, description = "Template Plaeceholder was created", body = OkI32Response),
    )
)]
async fn create_template_placeholder(
    ValidJson(request): ValidJson<TemplatePlaceholderForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let placeholder_id = TemplatePlaceholderService::create(request).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(placeholder_id),
    }))
}

#[utoipa::path(
    patch,
    request_body = TemplatePlaeholderForUpdateRequest,
    params  (
        ("placeholder_id" = String, Path, description = "Template Plaeceholder Id"),
    ),
    path = "/template-placeholders/{translation_id}",
    tag = TAG,
    description = "Change Email Template",
    responses(
        (status = 200, description= "Template Plaeceholder was updated", body= OkI32Response),
    )
)]
async fn update_template_placeholder(
    Path(placeholder_id): Path<i32>,
    ValidJson(request): ValidJson<TemplatePlaeholderForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    TemplatePlaceholderService::update(placeholder_id, request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/template-placeholders/{placeholder_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Template Plaeceholder was delete", body = OkI32Response),
    )
)]
async fn delete_template_placeholder(
    Path(placeholder_id): Path<i32>,
) -> Result<ResponseJson<OkUuid>> {
    TemplatePlaceholderService::delete(placeholder_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/template-placeholders/{placeholder_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Template Plaeceholder Data", body = TemplatePlaceholderDataResponse),
    )
)]
async fn get_template_placeholder(
    Path(placeholder_id): Path<i32>,
) -> Result<ResponseJson<TemplatePlaceholderData>> {
    let scope = TemplatePlaceholderService::get(placeholder_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/template-placeholders",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Template Plaeceholder", body = QueryResultResponse<TemplatePlaceholderData>),
    )
)]
async fn filter_template_placeholder(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<TemplatePlaceholderDataFilterParams>,
) -> Result<ResponseJson<QueryResult<TemplatePlaceholderData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = TemplatePlaceholderService::search(&pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<EmailTemplateCacheState>) -> Router {
    Router::new()
        .route("/template-placeholders", post(create_template_placeholder))
        .route(
            "/template-placeholders/{placeholder_id}",
            patch(update_template_placeholder),
        )
        .route(
            "/template-placeholders/{placeholder_id}",
            delete(delete_template_placeholder),
        )
        .route(
            "/template-placeholders/{placeholder_id}",
            get(get_template_placeholder),
        )
        .route("/template-placeholders", get(filter_template_placeholder))
        .with_state(app_state.clone())
}
