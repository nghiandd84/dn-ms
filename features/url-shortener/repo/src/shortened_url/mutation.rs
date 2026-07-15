use shared_shared_macro::Mutation;

use features_url_shortener_entities::shortened_url::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, ShortenedUrlForCreateDto,
    ShortenedUrlForUpdateDto,
};

use crate::shortened_url::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct ShortenedUrlMutationManager {}

pub struct ShortenedUrlMutation;

impl ShortenedUrlMutation {
    pub fn create_shortened_url<'a>(
        data: ShortenedUrlForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        ShortenedUrlMutationManager::create_uuid(data.into())
    }

    pub fn update_shortened_url<'a>(
        id: Uuid,
        data: ShortenedUrlForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ShortenedUrlMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete_shortened_url<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        ShortenedUrlMutationManager::delete_by_id_uuid(id)
    }
}
