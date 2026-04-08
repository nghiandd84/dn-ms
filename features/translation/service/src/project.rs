use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_translation_entities::project::ProjectForCreateDto;
use features_translation_model::{ProjectData, ProjectForCreateRequest, ProjectForUpdateRequest};
use features_translation_repo::project::{ProjectMutation, ProjectQuery};

pub struct ProjectService {}

impl ProjectService {
    pub async fn create_project<'a>(
        project_request: ProjectForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: ProjectForCreateDto = project_request.into();
        let project_id = ProjectMutation::create_project(dto).await;
        let id = match project_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating project: {:?}", e);
                return Err(AppError::Internal("Failed to create project".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_project_by_id<'a>(project_id: Uuid) -> Result<ProjectData, AppError> {
        ProjectQuery::get_project_by_id(project_id).await
    }

    pub async fn get_projects<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<ProjectData>, AppError> {
        ProjectQuery::get_projects(pagination, order, filters).await
    }

    pub async fn update_project<'a>(
        project_id: Uuid,
        project_request: ProjectForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto: features_translation_entities::project::ProjectForUpdateDto =
            project_request.into();
        let result = ProjectMutation::update_project(project_id, dto).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating project: {:?}", e);
                Err(AppError::Internal("Failed to update project".to_string()))
            }
        }
    }

    pub async fn delete_project<'a>(project_id: Uuid) -> Result<bool, AppError> {
        let result = ProjectMutation::delete_project(project_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting project: {:?}", e);
                Err(AppError::Internal("Failed to delete project".to_string()))
            }
        }
    }
}
