use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_translation_entities::key_tag::{ActiveModel, Column, Entity, ModelOptionDto};
use features_translation_model::key_tag::KeyTagData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
pub struct KeyTagQueryManager;

impl KeyTagQueryManager {
    pub async fn get_tags_by_key_id(key_id: Uuid) -> Result<Vec<KeyTagData>, AppError> {
        let paging = Pagination::new(1, 100); // Assuming a reasonable limit for tags
        let order = Order::default();
        let key_id_filter = FilterEnum::Uuid(FilterParam {
            name: Column::KeyId.to_string(),
            value: Some(key_id),
            raw_value: key_id.to_string(),
            operator: FilterOperator::Equal,
        });

        let filters: Vec<FilterEnum> = vec![key_id_filter];
        let query_result = Self::filter(&paging, &order, &FilterCondition::from(&filters)).await?;
        if query_result.result.is_empty() {
            return Err(AppError::EntityNotFound {
                entity: "tag_key".to_string(),
            });
        }
        let mapped_result = QueryResult {
            total_page: query_result.total_page,
            result: query_result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result.result)
    }

    pub async fn key_tag_exists(key_id: Uuid, tag_id: Uuid) -> Result<bool, AppError> {
        let paging = Pagination::new(1, 100); // Assuming a reasonable limit for tags
        let order = Order::default();
        let key_id_filter = FilterEnum::Uuid(FilterParam {
            name: Column::KeyId.to_string(),
            value: Some(key_id),
            raw_value: key_id.to_string(),
            operator: FilterOperator::Equal,
        });

        let tag_id_filter = FilterEnum::Uuid(FilterParam {
            name: Column::TagId.to_string(),
            value: Some(tag_id),
            raw_value: tag_id.to_string(),
            operator: FilterOperator::Equal,
        });

        let filters: Vec<FilterEnum> = vec![key_id_filter, tag_id_filter];
        let query_result = Self::filter(&paging, &order, &FilterCondition::from(&filters)).await;
        if query_result.is_err() {
            return Err(AppError::Internal("Failed to query key tags".to_string()));
        }
        let query_result = query_result.unwrap();

        Ok(query_result.result.len() > 0)
    }
}
