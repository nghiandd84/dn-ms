use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::token::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::token::TokenData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct TokenQueryManager;



pub struct TokenQuery {}

impl TokenQuery {
    pub async fn get<'a>(id: Uuid) -> Result<TokenData, DbErr> {
        let model = TokenQueryManager::get_by_id_uuid(id).await?;
        let user_data: TokenData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<TokenData>, DbErr> {
        let result = TokenQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
