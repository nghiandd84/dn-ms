use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_translation_entities::translation_version::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_translation_model::TranslationVersionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TranslationVersionQueryManager;



pub struct TranslationVersionQuery;

impl TranslationVersionQuery {
    pub async fn get_translation_version_by_id(
        version_id: Uuid,
    ) -> Result<TranslationVersionData, AppError> {
        let model = TranslationVersionQueryManager::get_by_id_uuid(version_id).await?;
        Ok(model.into())
    }

    pub async fn get_translation_versions<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        let result = TranslationVersionQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_latest_version_by_key_locale(
        key_id: Uuid,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        let key_param: FilterParam<Uuid> = FilterParam {
            name: Column::KeyId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(key_id),
            raw_value: key_id.to_string(),
        };
        let key_filter = FilterEnum::Uuid(key_param);

        let mut search_filters = filters.clone();
        search_filters.push(key_filter);

        let result =
            TranslationVersionQueryManager::filter(&pagination, &order, &search_filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
