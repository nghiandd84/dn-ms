use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_payments_stripe_entities::stripe_payment_intent::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, StripePaymentIntentForCreateDto,
    StripePaymentIntentForUpdateDto,
};

use crate::stripe_payment_intent::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct StripePaymentIntentMutationManager {}

pub struct StripePaymentIntentMutation;

impl StripePaymentIntentMutation {
    pub fn create_payment_intent<'a>(
        data: StripePaymentIntentForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        StripePaymentIntentMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_payment_intents<'a>(
        data: Vec<StripePaymentIntentForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        StripePaymentIntentMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_payment_intent<'a>(
        payment_intent_id: Uuid,
        data: StripePaymentIntentForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        StripePaymentIntentMutationManager::update_by_id_uuid(payment_intent_id, data.into())
    }

    pub fn bulk_update_payment_intents<'a>(
        data: Vec<(Uuid, StripePaymentIntentForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        StripePaymentIntentMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_payment_intent<'a>(
        payment_intent_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        StripePaymentIntentMutationManager::delete_by_id_uuid(payment_intent_id)
    }
}
