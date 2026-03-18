use shared_shared_macro::Mutation;

use features_profiles_entities::user_preference::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, UserPreferenceForCreateDto,
    UserPreferenceForUpdateDto,
};

use crate::user_preference::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct UserPreferenceMutationManager {}

pub struct UserPreferenceMutation {}

impl UserPreferenceMutation {
    pub fn create_user_preference<'a>(
        data: UserPreferenceForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        UserPreferenceMutationManager::create_uuid(data.into())
    }

    pub fn update_user_preference<'a>(
        preference_id: Uuid,
        data: UserPreferenceForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserPreferenceMutationManager::update_by_id_uuid(preference_id, data.into())
    }

    pub fn delete_user_preference<'a>(
        preference_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        UserPreferenceMutationManager::delete_by_id_uuid(preference_id)
    }
}
