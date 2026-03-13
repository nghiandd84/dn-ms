use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_payments_core_entities::payment_method_limit::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaymentMethodLimitForCreateDto,
    PaymentMethodLimitForUpdateDto,
};

use crate::payment_method_limit::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaymentMethodLimitMutationManager {}

pub struct PaymentMethodLimitMutation;

impl PaymentMethodLimitMutation {
    pub fn create_payment_method_limit<'a>(
        data: PaymentMethodLimitForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        PaymentMethodLimitMutationManager::create_uuid(data.into())
    }

    pub fn update_payment_method_limit<'a>(
        limit_id: Uuid,
        data: PaymentMethodLimitForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMethodLimitMutationManager::update_by_id_uuid(limit_id, data.into())
    }

    pub fn delete_payment_method_limit<'a>(
        limit_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMethodLimitMutationManager::delete_by_id_uuid(limit_id)
    }
}
