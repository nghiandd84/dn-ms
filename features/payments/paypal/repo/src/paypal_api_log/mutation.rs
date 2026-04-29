use shared_shared_macro::Mutation;

use features_payments_paypal_entities::paypal_api_log::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, PaypalApiLogForCreateDto,
    PaypalApiLogForUpdateDto,
};

use crate::paypal_api_log::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct PaypalApiLogMutationManager {}

pub struct PaypalApiLogMutation;

impl PaypalApiLogMutation {
    pub fn create_api_log<'a>(
        data: PaypalApiLogForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, sea_orm::DbErr>> + 'a {
        PaypalApiLogMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_api_logs<'a>(
        data: Vec<PaypalApiLogForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalApiLogMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_api_log<'a>(
        api_log_id: Uuid,
        data: PaypalApiLogForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalApiLogMutationManager::update_by_id_uuid(api_log_id, data.into())
    }

    pub fn bulk_update_api_logs<'a>(
        data: Vec<(Uuid, PaypalApiLogForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, sea_orm::DbErr>> + 'a {
        PaypalApiLogMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_api_log<'a>(
        api_log_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, sea_orm::DbErr>> + 'a {
        PaypalApiLogMutationManager::delete_by_id_uuid(api_log_id)
    }
}
