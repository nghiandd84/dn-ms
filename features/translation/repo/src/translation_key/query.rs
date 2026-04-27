use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_translation_entities::translation_key::{ActiveModel, Column, Entity, ModelOptionDto};
use features_translation_model::TranslationKeyData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TranslationKeyQueryManager;



pub struct TranslationKeyQuery;

impl TranslationKeyQuery {
    pub async fn get_translation_key_by_id(key_id: Uuid) -> Result<TranslationKeyData, AppError> {
        let model = TranslationKeyQueryManager::get_by_id_uuid(key_id).await?;
        Ok(model.into())
    }

    pub async fn get_translation_keys_by_project<'a>(
        project_id: Uuid,
    ) -> Result<QueryResult<TranslationKeyData>, AppError> {
        let pagination = Pagination::new(1, 1000);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::ProjectId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(project_id),
            raw_value: project_id.to_string(),
        };
        let project_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![project_filter];

        let result = TranslationKeyQueryManager::filter(&pagination, &order, &filters).await?;

        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_translation_keys<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TranslationKeyData>, AppError> {
        let result = TranslationKeyQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
