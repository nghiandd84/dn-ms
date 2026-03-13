use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use tracing::{instrument, Level};
use uuid::Uuid;

use features_translation_model::{
    state::{TranslationAppState, TranslationCacheState},
    ProjectData, ProjectDataFilterParams, ProjectForCreateRequest, ProjectForUpdateRequest,
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

use features_translation_service::ProjectService;

const TAG: &str = "project";

#[utoipa::path(
    post,
    path = "/projects",
    tag = TAG,
    request_body = ProjectForCreateRequest,
    responses(
        (status = 201, description = "Project created successfully", body = OkUuidResponse),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn create_project(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Json(req): Json<ProjectForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let project_id = ProjectService::create_project(req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(project_id),
    }))
}

#[utoipa::path(
    get,
    path = "/projects/{project_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Project retrieved successfully", body = ProjectData),
    )
)]
async fn get_project(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(project_id): Path<Uuid>,
) -> Result<ResponseJson<ProjectData>> {
    let project = ProjectService::get_project_by_id(&state.conn, project_id).await?;
    Ok(ResponseJson(project))
}

#[utoipa::path(
    get,
    path = "/projects",
    tag = TAG,
    params(
        Order,
        Pagination
    ),
    responses(
        (status = 200, description = "Filtered projects", body = QueryResultResponse<ProjectData>),
    )
)]
#[instrument(level = Level::INFO, skip_all)]
async fn filter_projects(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    query_pagination: Query<Pagination>,
    query_order: Query<Order>,
    filter_params: FilterParams<ProjectDataFilterParams>,
) -> Result<ResponseJson<QueryResult<ProjectData>>> {
    let pagination = query_pagination.0;
    let order = query_order.0;
    let filters = filter_params.0.all_filters();
    let result = ProjectService::get_projects(&state.conn, &pagination, &order, &filters).await?;
    Ok(ResponseJson(result))
}

#[utoipa::path(
    patch,
    path = "/projects/{project_id}",
    tag = TAG,
    request_body = ProjectForUpdateRequest,
    responses(
        (status = 200, description = "Project updated successfully", body = OkUuidResponse),
    )
)]
async fn update_project(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(project_id): Path<Uuid>,
    Json(req): Json<ProjectForUpdateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    ProjectService::update_project(project_id, req).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(project_id),
    }))
}

#[utoipa::path(
    delete,
    path = "/projects/{project_id}",
    tag = TAG,
    responses(
        (status = 200, description = "Project deleted successfully", body = OkUuidResponse),
    )
)]
async fn delete_project(
    state: State<AppState<TranslationAppState, TranslationCacheState>>,
    Path(project_id): Path<Uuid>,
) -> Result<ResponseJson<OkUuid>> {
    ProjectService::delete_project(project_id).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(project_id),
    }))
}

pub fn routes(app_state: &AppState<TranslationAppState, TranslationCacheState>) -> Router {
    Router::new()
        .route("/projects", post(create_project))
        .route("/projects", get(filter_projects))
        .route("/projects/{project_id}", get(get_project))
        .route("/projects/{project_id}", patch(update_project))
        .route("/projects/{project_id}", delete(delete_project))
        .with_state(app_state.clone())
}
