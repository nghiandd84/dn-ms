use shared_shared_macro::Mutation;

use features_event_entities::event::{
    ActiveModel, Column, Entity, EventForCreateDto, EventForUpdateDto, Model, ModelOptionDto,
};

use crate::event::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct EventMutationManager {}

pub struct EventMutation;

impl EventMutation {
    pub fn create_event<'a>(
        data: EventForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        EventMutationManager::create_uuid(data.into())
    }

    pub fn update_event<'a>(
        event_id: Uuid,
        data: EventForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EventMutationManager::update_by_id_uuid(event_id, data.into())
    }

    pub fn delete_event<'a>(
        event_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        EventMutationManager::delete_by_id_uuid(event_id)
    }
}
