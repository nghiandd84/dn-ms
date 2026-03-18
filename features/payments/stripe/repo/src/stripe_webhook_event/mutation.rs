use shared_shared_macro::Mutation;

use features_payments_stripe_entities::stripe_webhook_event::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, StripeWebhookEventForCreateDto,
    StripeWebhookEventForUpdateDto,
};

use crate::stripe_webhook_event::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct StripeWebhookEventMutationManager {}

pub struct StripeWebhookEventMutation;

impl StripeWebhookEventMutation {
    pub fn create_webhook_event<'a>(
        data: StripeWebhookEventForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        StripeWebhookEventMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_webhook_events<'a>(
        data: Vec<StripeWebhookEventForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        StripeWebhookEventMutationManager::bulk_create_uuid(
            data.into_iter().map(|d| d.into()).collect(),
        )
    }

    pub fn update_webhook_event<'a>(
        webhook_event_id: Uuid,
        data: StripeWebhookEventForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        StripeWebhookEventMutationManager::update_by_id_uuid(webhook_event_id, data.into())
    }

    pub fn bulk_update_webhook_events<'a>(
        data: Vec<(Uuid, StripeWebhookEventForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        StripeWebhookEventMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_webhook_event<'a>(
        webhook_event_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        StripeWebhookEventMutationManager::delete_by_id_uuid(webhook_event_id)
    }
}
