use shared_shared_macro::Mutation;

use features_url_shortener_entities::url_click::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, UrlClickForCreateDto,
};

use crate::url_click::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct UrlClickMutationManager {}

pub struct UrlClickMutation;

impl UrlClickMutation {
    pub fn record_click<'a>(
        data: UrlClickForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        UrlClickMutationManager::create_uuid(data.into())
    }
}
