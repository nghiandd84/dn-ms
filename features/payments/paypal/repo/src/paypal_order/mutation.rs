use shared_shared_macro::Mutation;

use features_payments_paypal_entities::paypal_order::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaypalOrderForCreateDto,
    PaypalOrderForUpdateDto,
};

use crate::paypal_order::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaypalOrderMutationManager {}

pub struct PaypalOrderMutation;

impl PaypalOrderMutation {
    pub fn create_order<'a>(
        data: PaypalOrderForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        PaypalOrderMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_orders<'a>(
        data: Vec<PaypalOrderForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalOrderMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_order<'a>(
        order_id: Uuid,
        data: PaypalOrderForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalOrderMutationManager::update_by_id_uuid(order_id, data.into())
    }

    pub fn bulk_update_orders<'a>(
        data: Vec<(Uuid, PaypalOrderForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalOrderMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_order<'a>(
        order_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalOrderMutationManager::delete_by_id_uuid(order_id)
    }
}
