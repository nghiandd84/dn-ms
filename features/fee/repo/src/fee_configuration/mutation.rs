use shared_shared_macro::Mutation;

use features_fee_entities::fee_configuration::{
    ActiveModel, Column, Entity, FeeConfigurationForCreateDto, FeeConfigurationForUpdateDto, Model,
    ModelOptionDto,
};

use crate::fee_configuration::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct FeeConfigurationMutationManager {}

impl FeeConfigurationMutationManager {}

pub struct FeeConfigurationMutation;

impl FeeConfigurationMutation {
    pub fn create_fee_configuration<'a>(
        data: FeeConfigurationForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        FeeConfigurationMutationManager::create_uuid(data.into())
    }

    pub fn bulk_create_fee_configurations<'a>(
        data: Vec<FeeConfigurationForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        FeeConfigurationMutationManager::bulk_create_uuid(
            data.into_iter().map(|d| d.into()).collect(),
        )
    }

    pub fn update_fee_configuration<'a>(
        fee_configuration_id: Uuid,
        data: FeeConfigurationForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        FeeConfigurationMutationManager::update_by_id_uuid(fee_configuration_id, data.into())
    }

    pub fn bulk_update_fee_configurations<'a>(
        data: Vec<(Uuid, FeeConfigurationForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>> + 'a {
        FeeConfigurationMutationManager::bulk_update_by_id_uuid(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_fee_configuration<'a>(
        fee_configuration_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        FeeConfigurationMutationManager::delete_by_id_uuid(fee_configuration_id)
    }
}
