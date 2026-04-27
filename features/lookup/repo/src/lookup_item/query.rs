use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_lookup_entities::lookup_item::{ActiveModel, Column, Entity, ModelOptionDto};
use features_lookup_model::lookup_item::LookupItemData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct LookupItemQueryManager;



pub struct LookupItemQuery;

impl LookupItemQuery {
    pub async fn get_lookup_item_by_id(id: Uuid) -> Result<LookupItemData, AppError> {
        let model = LookupItemQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_lookup_items<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<LookupItemData>, AppError> {
        let result = LookupItemQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_lookup_items_by_type(
        lookup_type_id: Uuid,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<LookupItemData>, AppError> {
        let type_param: FilterParam<Uuid> = FilterParam {
            name: Column::LookupTypeId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(lookup_type_id),
            raw_value: lookup_type_id.to_string(),
        };
        let mut filters = filters.clone();
        filters.push(FilterEnum::Uuid(type_param));

        let result = LookupItemQueryManager::filter(pagination, order, &filters).await?;
        debug!("Raw query result: {:?}", result);
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };

        Ok(mapped_result)
    }
}
