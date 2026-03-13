use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_payments_core_entities::payment_attempt::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaymentAttemptForCreateDto,
    PaymentAttemptForUpdateDto,
};

use crate::payment_attempt::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaymentAttemptMutationManager {}

pub struct PaymentAttemptMutation;

impl PaymentAttemptMutation {
    pub fn create_payment_attempt<'a>(
        data: PaymentAttemptForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        PaymentAttemptMutationManager::create_uuid(data.into())
    }

    pub fn update_payment_attempt<'a>(
        attempt_id: Uuid,
        data: PaymentAttemptForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentAttemptMutationManager::update_by_id_uuid(attempt_id, data.into())
    }

    pub fn delete_payment_attempt<'a>(
        attempt_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentAttemptMutationManager::delete_by_id_uuid(attempt_id)
    }
}
