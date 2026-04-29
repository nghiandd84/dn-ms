use shared_shared_macro::Mutation;

use features_payments_paypal_entities::paypal_refund::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaypalRefundForCreateDto,
    PaypalRefundForUpdateDto,
};

use crate::paypal_refund::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaypalRefundMutationManager {}

pub struct PaypalRefundMutation;

impl PaypalRefundMutation {
    pub fn create_refund<'a>(
        data: PaypalRefundForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        PaypalRefundMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_refunds<'a>(
        data: Vec<PaypalRefundForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalRefundMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_refund<'a>(
        refund_id: Uuid,
        data: PaypalRefundForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalRefundMutationManager::update_by_id_uuid(refund_id, data.into())
    }

    pub fn bulk_update_refunds<'a>(
        data: Vec<(Uuid, PaypalRefundForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalRefundMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_refund<'a>(
        refund_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalRefundMutationManager::delete_by_id_uuid(refund_id)
    }
}
