use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_profiles_entities::social_link::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, SocialLinkForCreateDto,
    SocialLinkForUpdateDto,
};

use crate::social_link::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct SocialLinkMutationManager {}

pub struct SocialLinkMutation {}

impl SocialLinkMutation {
    pub fn create_social_link<'a>(
        data: SocialLinkForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        SocialLinkMutationManager::create_uuid(data.into())
    }

    pub fn update_social_link<'a>(
        link_id: Uuid,
        data: SocialLinkForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        SocialLinkMutationManager::update_by_id_uuid(link_id, data.into())
    }

    pub fn delete_social_link<'a>(
        link_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        SocialLinkMutationManager::delete_by_id_uuid(link_id)
    }
}
