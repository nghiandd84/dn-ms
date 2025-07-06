use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::scope::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::scope::ScopeData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ScopeQueryManager;

impl ScopeQueryManager {
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

pub struct ScopeQuery {}

impl ScopeQuery {
    pub async fn get<'a>(db: &'a DbConn, id: Uuid) -> Result<ScopeData, DbErr> {
        let model = ScopeQueryManager::get_by_id_uuid(db, id).await?;
        let user_data: ScopeData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<ScopeData>, DbErr> {
        let result = ScopeQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
