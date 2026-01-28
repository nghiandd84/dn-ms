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

impl TranslationVersionQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct TranslationVersionQuery;

impl TranslationVersionQuery {
    pub async fn get_translation_version_by_id(
        db: &DbConn,
        version_id: Uuid,
    ) -> Result<TranslationVersionData, AppError> {
        let model = TranslationVersionQueryManager::get_by_id_uuid(db, version_id).await?;
        Ok(model.into())
    }

    pub async fn get_translation_versions<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        let result = TranslationVersionQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }

    pub async fn get_latest_version_by_key_locale(
        db: &DbConn,
        key_id: Uuid,
        locale: &str,
    ) -> Result<QueryResult<TranslationVersionData>, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let key_param: FilterParam<Uuid> = FilterParam {
            name: Column::KeyId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(key_id),
            raw_value: key_id.to_string(),
        };
        let key_filter = FilterEnum::Uuid(key_param);

        let locale_param: FilterParam<String> = FilterParam {
            name: Column::Locale.to_string(),
            operator: FilterOperator::Equal,
            value: Some(locale.to_string()),
            raw_value: locale.to_string(),
        };
        let locale_filter = FilterEnum::String(locale_param);

        let filters: Vec<FilterEnum> = vec![key_filter, locale_filter];

        let result =
            TranslationVersionQueryManager::filter(db, &pagination, &order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
