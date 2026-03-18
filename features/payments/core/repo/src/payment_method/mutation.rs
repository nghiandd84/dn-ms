use shared_shared_macro::Mutation;

use features_payments_core_entities::payment_method::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaymentMethodForCreateDto,
    PaymentMethodForUpdateDto,
};

use crate::payment_method::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaymentMethodMutationManager {}

pub struct PaymentMethodMutation;

impl PaymentMethodMutation {
    pub fn create_payment_method<'a>(
        data: PaymentMethodForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        PaymentMethodMutationManager::create_uuid(data.into())
    }

    pub fn update_payment_method<'a>(
        method_id: Uuid,
        data: PaymentMethodForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMethodMutationManager::update_by_id_uuid(method_id, data.into())
    }

    pub fn delete_payment_method<'a>(
        method_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMethodMutationManager::delete_by_id_uuid(method_id)
    }
}
