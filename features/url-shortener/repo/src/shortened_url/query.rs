use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_url_shortener_entities::shortened_url::{ActiveModel, Column, Entity, ModelOptionDto};
use features_url_shortener_model::shortened_url::ShortenedUrlData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ShortenedUrlQueryManager;

pub struct ShortenedUrlQuery;

impl ShortenedUrlQuery {
    pub async fn get_by_id(
        id: Uuid,
        _query_params: &QueryParams,
    ) -> Result<ShortenedUrlData, AppError> {
        let model = ShortenedUrlQueryManager::get_by_id_uuid(id).await?;
        Ok(model.into())
    }

    pub async fn get_by_short_code(code: &str) -> Result<ShortenedUrlData, AppError> {
        let code_param: FilterParam<String> = FilterParam {
            name: Column::ShortCode.to_string(),
            operator: FilterOperator::Equal,
            value: Some(code.to_string()),
            raw_value: code.to_string(),
        };
        let filters: FilterCondition = vec![FilterEnum::String(code_param)].into();
        let pagination = Pagination::default();
        let order = Order::default();

        let result = ShortenedUrlQueryManager::filter(&pagination, &order, &filters).await?;
        let item = result
            .result
            .into_iter()
            .next()
            .ok_or(AppError::EntityNotFound {
                entity: format!("shortened_url code {}", code),
            })?;
        Ok(item.into())
    }

    pub async fn get_user_urls(
        user_id: &Uuid,
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ShortenedUrlData>, AppError> {
        let mut filters = filters.clone();
        filters.push_leaf(FilterEnum::Uuid(FilterParam {
            name: Column::UserId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(*user_id),
            raw_value: user_id.to_string(),
        }));

        let result = ShortenedUrlQueryManager::filter(pagination, order, &filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
