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
    email_template::{
        EmailTemplateData, EmailTemplateDataFilterParams, EmailTemplateDataResponse,
        EmailTemplateForCreateRequest, EmailTemplateForUpdateRequest,
    },
    state::EmailTemplateCacheState,
};
use features_email_template_service::EmailTemplateService;

const TAG: &str = "Email-Template";

#[utoipa::path(
    post,
    request_body = EmailTemplateForCreateRequest,
    path = "/email-templates",
    tag = TAG,
    responses(
        (status = 200, description = "Client is created", body = OkI32Response),
    )
)]
async fn create_email_template(
    ValidJson(request): ValidJson<EmailTemplateForCreateRequest>,
) -> Result<ResponseJson<OkI32>> {
    let template_id = EmailTemplateService::create(request).await?;
    Ok(ResponseJson(OkI32 {
        ok: true,
        id: Some(template_id),
    }))
}

#[utoipa::path(
    patch,
    request_body = EmailTemplateForUpdateRequest,
    params  (
        ("template_id" = String, Path, description = "Email Template Id"),
    ),
    path = "/email-templates/{template_id}",
    tag = TAG,
    description = "Change Email Template",
    responses(
        (status = 200, description= "Email Template was updated", body= OkI32Response),
    )
)]
async fn update_email_template(
    Path(template_id): Path<i32>,
    ValidJson(request): ValidJson<EmailTemplateForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    EmailTemplateService::update(template_id, request.into()).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    delete,
    path = "/email-templates/{template_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Email Template was delete", body = OkI32Response),
    )
)]
async fn delete_email_template(Path(template_id): Path<i32>) -> Result<ResponseJson<OkUuid>> {
    EmailTemplateService::delete(template_id).await?;
    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

#[utoipa::path(
    get,
    path = "/email-templates/{template_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Email Template Data", body = EmailTemplateDataResponse),
    )
)]
async fn get_email_template(
    Path(template_id): Path<i32>,
) -> Result<ResponseJson<EmailTemplateData>> {
    let scope = EmailTemplateService::get(template_id).await?;
    Ok(ResponseJson(scope))
}

#[utoipa::path(
    get,
    path = "/email-templates",
    tag = TAG,
    params  (
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered Email Template", body = QueryResultResponse<EmailTemplateData>),
    )
)]
async fn filter_email_template(
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter: Query<EmailTemplateDataFilterParams>,
) -> Result<ResponseJson<QueryResult<EmailTemplateData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let all_filters = filter.0.all_filters();

    let result = EmailTemplateService::search(&pagination, &order, &all_filters).await?;
    debug!("{:?}", result);
    Ok(ResponseJson(result))
}

pub fn routes(app_state: &AppState<EmailTemplateCacheState>) -> Router {
    Router::new()
        .route("/email-templates", post(create_email_template))
        .route(
            "/email-templates/{template_id}",
            patch(update_email_template),
        )
        .route(
            "/email-templates/{template_id}",
            delete(delete_email_template),
        )
        .route("/email-templates/{template_id}", get(get_email_template))
        .route("/email-templates", get(filter_email_template))
        .with_state(app_state.clone())
}
