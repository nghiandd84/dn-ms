use shared_shared_macro::Mutation;

use features_payments_paypal_entities::paypal_webhook_event::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaypalWebhookEventForCreateDto,
    PaypalWebhookEventForUpdateDto,
};

use crate::paypal_webhook_event::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaypalWebhookEventMutationManager {}

pub struct PaypalWebhookEventMutation;

impl PaypalWebhookEventMutation {
    pub fn create_webhook_event<'a>(
        data: PaypalWebhookEventForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        PaypalWebhookEventMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_webhook_events<'a>(
        data: Vec<PaypalWebhookEventForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalWebhookEventMutationManager::bulk_create_uuid(
            data.into_iter().map(|d| d.into()).collect(),
        )
    }

    pub fn update_webhook_event<'a>(
        webhook_event_id: Uuid,
        data: PaypalWebhookEventForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalWebhookEventMutationManager::update_by_id_uuid(webhook_event_id, data.into())
    }

    pub fn bulk_update_webhook_events<'a>(
        data: Vec<(Uuid, PaypalWebhookEventForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalWebhookEventMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_webhook_event<'a>(
        webhook_event_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalWebhookEventMutationManager::delete_by_id_uuid(webhook_event_id)
    }
}
