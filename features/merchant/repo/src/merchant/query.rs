use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_merchant_entities::merchant::{ActiveModel, Column, Entity, ModelOptionDto};
use features_merchant_model::merchant::MerchantData;

#[derive(Query)]
#[query(key_type(String))]
#[query_filter(column_name(Column))]
struct MerchantQueryManager;

pub struct MerchantQuery;

impl MerchantQuery {
    pub async fn get_merchant_by_id(merchant_id: String) -> Result<MerchantData, AppError> {
        let model = MerchantQueryManager::get_by_id_str(merchant_id).await?;
        Ok(model.into())
    }

    pub async fn get_merchants<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<MerchantData>, AppError> {
        let result = MerchantQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
