use shared_shared_macro::Mutation;

use features_merchant_entities::merchant::{
    ActiveModel, Column, Entity, MerchantForCreateDto, MerchantForUpdateDto, Model, ModelOptionDto,
};

use crate::merchant::util::assign;

#[derive(Mutation)]
#[mutation(key_type(String))]
struct MerchantMutationManager {}

impl MerchantMutationManager {
    /*

    async fn bulk_create_str1(models: Vec<Model>) -> Result<Vec<String>, DbErr> {
        let active_models: Vec<ActiveModel> = models
            .into_iter()
            .map(|model| {
                let mut active_model: ActiveModel = model.into();
                active_model.not_set(Column::Id);
                active_model
            })
            .collect();

        let result_models = Entity::insert_many(active_models)
            .exec_with_returning(Self::get_db())
            .await?;
        let ids = result_models.iter().map(|model| model.id.clone()).collect();
        Ok(ids)
    }
    */
}

pub struct MerchantMutation;

impl MerchantMutation {
    pub fn create_merchant<'a>(
        data: MerchantForCreateDto,
    ) -> impl std::future::Future<Output = Result<String, DbErr>> + 'a {
        MerchantMutationManager::create_str(data.into())
    }
    pub fn bulk_create_merchants<'a>(
        data: Vec<MerchantForCreateDto>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>> + 'a {
        MerchantMutationManager::bulk_create_str(data.into_iter().map(|d| d.into()).collect())
    }
    /*

    pub fn update_merchant<'a>(
        merchant_id: String,
        data: MerchantForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        MerchantMutationManager::update_by_id_string(merchant_id, data.into())
    }

    pub fn bulk_update_merchants<'a>(
        data: Vec<(String, MerchantForUpdateDto)>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>> + 'a {
        MerchantMutationManager::bulk_update_by_id_string(
            data.into_iter().map(|(id, dto)| (id, dto.into())).collect(),
        )
    }

    pub fn delete_merchant<'a>(
        merchant_id: String,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        MerchantMutationManager::delete_by_id_string(merchant_id)
    }
     */
}
