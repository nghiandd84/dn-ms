use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::active_code::{ActiveModel, Column, Entity, ModelOptionDto};

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ActiveCodeQueryManager;

pub struct ActiveCodeQuery {}

impl ActiveCodeQuery {
    pub async fn search(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ModelOptionDto>, DbErr> {
        ActiveCodeQueryManager::filter(pagination, order, filters).await
    }
}
