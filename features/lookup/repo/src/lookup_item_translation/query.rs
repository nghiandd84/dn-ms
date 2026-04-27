use uuid::Uuid;

use features_lookup_entities::lookup_item_translation::{
    ActiveModel, Column, Entity, ModelOptionDto,
};
use features_lookup_model::lookup_item_translation::LookupItemTranslationData;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct LookupItemTranslationQueryManager;

pub struct LookupItemTranslationQuery;

impl LookupItemTranslationQuery {
    pub async fn get_translation_by_id(id: Uuid) -> Result<LookupItemTranslationData, AppError> {
        let model = LookupItemTranslationQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_translations(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<LookupItemTranslationData>, AppError> {
        let result = LookupItemTranslationQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_translations_by_item_id(
        lookup_item_id: Uuid,
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<LookupItemTranslationData>, AppError> {
        let lookup_type_param: FilterParam<Uuid> = FilterParam {
            name: Column::LookupItemId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(lookup_item_id),
            raw_value: lookup_item_id.to_string(),
        };
        let mut filters = filters.clone();
        filters.push_leaf(FilterEnum::Uuid(lookup_type_param));

        let result =
            LookupItemTranslationQueryManager::filter(&pagination, &order, &filters).await?;

        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_translation_by_item_id_locale(
        lookup_item_id: Uuid,
        locale: &str,
    ) -> Result<LookupItemTranslationData, AppError> {
        let item_id_param: FilterParam<Uuid> = FilterParam {
            name: Column::LookupItemId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(lookup_item_id),
            raw_value: lookup_item_id.to_string(),
        };
        let locale_param: FilterParam<String> = FilterParam {
            name: Column::Locale.to_string(),
            operator: FilterOperator::Equal,
            value: Some(locale.to_string()),
            raw_value: locale.to_string(),
        };

        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let result = LookupItemTranslationQueryManager::filter(
            &pagination,
            &order,
            &FilterCondition::from(vec![
                FilterEnum::Uuid(item_id_param),
                FilterEnum::String(locale_param),
            ]),
        )
        .await?;

        let entry = result
            .result
            .into_iter()
            .next()
            .ok_or(AppError::EntityNotFound {
                entity: format!(
                    "lookup_item_translation for item={} locale={}",
                    lookup_item_id, locale
                ),
            })?;

        Ok(entry.into())
    }
}
