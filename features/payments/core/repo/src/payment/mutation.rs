use sea_orm::{DbConn, DbErr};
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_payments_core_entities::payment::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaymentForCreateDto, PaymentForUpdateDto,
};

use crate::payment::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaymentMutationManager {}

pub struct PaymentMutation;

impl PaymentMutation {
    pub fn create_payment<'a>(
        data: PaymentForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        PaymentMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_payments<'a>(
        data: Vec<PaymentForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        PaymentMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_payment<'a>(
        payment_id: Uuid,
        data: PaymentForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMutationManager::update_by_id_uuid(payment_id, data.into())
    }

    pub fn bulk_update_payments<'a>(
        data: Vec<(Uuid, PaymentForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        PaymentMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_payment<'a>(
        payment_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        PaymentMutationManager::delete_by_id_uuid(payment_id)
    }
}
