use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_translation_entities::project::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ProjectForCreateDto, ProjectForUpdateDto,
};

use crate::project::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ProjectMutationManager {}

pub struct ProjectMutation {}

impl ProjectMutation {
    pub fn create_project<'a>(
        data: ProjectForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ProjectMutationManager::create_uuid(data.into())
    }

    pub fn update_project<'a>(
        project_id: Uuid,
        data: ProjectForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ProjectMutationManager::update_by_id_uuid(project_id, data.into())
    }

    pub fn delete_project<'a>(
        project_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ProjectMutationManager::delete_by_id_uuid(project_id)
    }
}
