use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_profiles_entities::profile::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ProfileForCreateDto, ProfileForUpdateDto,
};

use crate::profile::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ProfileMutationManager {}

pub struct ProfileMutation {}

impl ProfileMutation {
    pub fn create_profile<'a>(
        data: ProfileForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ProfileMutationManager::create_uuid(data.into())
    }

    pub fn update_profile<'a>(
        profile_id: Uuid,
        data: ProfileForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ProfileMutationManager::update_by_id_uuid(profile_id, data.into())
    }

    pub fn delete_profile<'a>(
        profile_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ProfileMutationManager::delete_by_id_uuid(profile_id)
    }
}
