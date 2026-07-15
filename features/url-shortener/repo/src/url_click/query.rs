use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_url_shortener_entities::url_click::{ActiveModel, Column, Entity, ModelOptionDto};
use features_url_shortener_model::url_click::UrlClickData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct UrlClickQueryManager;

pub struct UrlClickQuery;

impl UrlClickQuery {
    pub async fn get_clicks_by_url_id(
        url_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<UrlClickData>, AppError> {
        let filters: FilterCondition = vec![FilterEnum::String(FilterParam {
            name: Column::UrlId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(url_id.to_string()),
            raw_value: url_id.to_string(),
        })]
        .into();

        let result = UrlClickQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
