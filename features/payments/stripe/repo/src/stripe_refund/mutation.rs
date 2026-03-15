use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_payments_stripe_entities::stripe_refund::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, StripeRefundForCreateDto,
    StripeRefundForUpdateDto,
};

use crate::stripe_refund::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct StripeRefundMutationManager {}

pub struct StripeRefundMutation;

impl StripeRefundMutation {
    pub fn create_refund<'a>(
        data: StripeRefundForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        StripeRefundMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_refunds<'a>(
        data: Vec<StripeRefundForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        StripeRefundMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_refund<'a>(
        refund_id: Uuid,
        data: StripeRefundForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        StripeRefundMutationManager::update_by_id_uuid(refund_id, data.into())
    }

    pub fn bulk_update_refunds<'a>(
        data: Vec<(Uuid, StripeRefundForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        StripeRefundMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_refund<'a>(refund_id: Uuid) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        StripeRefundMutationManager::delete_by_id_uuid(refund_id)
    }
}
