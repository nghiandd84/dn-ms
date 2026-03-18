use shared_shared_macro::Mutation;

use features_payments_stripe_entities::stripe_api_log::{
    ActiveModel, Column, Entity, Model, ModelOptionDto, StripeApiLogForCreateDto,
    StripeApiLogForUpdateDto,
};

use crate::stripe_api_log::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct StripeApiLogMutationManager {}

pub struct StripeApiLogMutation;

impl StripeApiLogMutation {
    pub fn create_api_log<'a>(
        data: StripeApiLogForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        StripeApiLogMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_api_logs<'a>(
        data: Vec<StripeApiLogForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        StripeApiLogMutationManager::bulk_create_uuid(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_api_log<'a>(
        api_log_id: Uuid,
        data: StripeApiLogForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        StripeApiLogMutationManager::update_by_id_uuid(api_log_id, data.into())
    }

    pub fn bulk_update_api_logs<'a>(
        data: Vec<(Uuid, StripeApiLogForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        StripeApiLogMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_api_log<'a>(
        api_log_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        StripeApiLogMutationManager::delete_by_id_uuid(api_log_id)
    }
}
