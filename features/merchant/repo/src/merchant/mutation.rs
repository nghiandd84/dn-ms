use shared_shared_macro::Mutation;

use features_merchant_entities::merchant::{
    ActiveModel, Column, Entity, MerchantForCreateDto, MerchantForUpdateDto, Model, ModelOptionDto,
};

use crate::merchant::util::assign;

#[derive(Mutation)]
#[mutation(key_type(String))]
struct MerchantMutationManager {}

impl MerchantMutationManager {}

pub struct MerchantMutation;

impl MerchantMutation {
    pub fn create_merchant<'a>(
        data: MerchantForCreateDto,
    ) -> impl std::future::Future<Output = Result<String, DbErr>> + 'a {
        let mut model: Model = data.into();
        let id = Uuid::new_v4().to_string();
        model.id = "mch_".to_string() + &id.replace("-", "")[..16];
        MerchantMutationManager::create_str(model)
    }
    pub fn bulk_create_merchants<'a>(
        data: Vec<MerchantForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>> + 'a {
        MerchantMutationManager::bulk_create_str(data.into_iter().map(|d| d.into()).collect())
    }

    pub fn update_merchant<'a>(
        merchant_id: String,
        data: MerchantForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        MerchantMutationManager::update_by_id_str(merchant_id, data.into())
    }

    pub fn bulk_update_merchants<'a>(
        data: Vec<(String, MerchantForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>> + 'a {
        MerchantMutationManager::bulk_update_by_id_str(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }
    pub fn delete_merchant<'a>(
        merchant_id: String,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        MerchantMutationManager::delete_by_id_str(merchant_id)
    }
}
