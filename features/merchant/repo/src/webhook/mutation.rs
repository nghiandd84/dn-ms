use shared_shared_macro::Mutation;

use features_merchant_entities::webhook::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, WebhookForCreateDto, WebhookForUpdateDto,
};

use crate::webhook::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct WebhookMutationManager {}

impl WebhookMutationManager {}

pub struct WebhookMutation;

impl WebhookMutation {
    pub fn create_webhook<'a>(
        data: WebhookForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        let mut model: Model = data.into();
        let id = Uuid::new_v4();
        model.id = id;
        WebhookMutationManager::create_uuid(model)
    }

    pub fn bulk_create_webhooks<'a>(
        data: Vec<WebhookForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        WebhookMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_webhook<'a>(
        webhook_id: Uuid,
        data: WebhookForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WebhookMutationManager::update_by_id_uuid(webhook_id, data.into())
    }

    pub fn bulk_update_webhooks<'a>(
        data: Vec<(Uuid, WebhookForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        WebhookMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_webhook<'a>(
        webhook_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        WebhookMutationManager::delete_by_id_uuid(webhook_id)
    }
}
