use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::permission::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::permission::PermissionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PermissionQueryManager;

impl PermissionQueryManager {
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

pub struct PermissionQuery {}

impl PermissionQuery {
    pub async fn get<'a>(db: &'a DbConn, id: Uuid) -> Result<PermissionData, DbErr> {
        let model = PermissionQueryManager::get_by_id_uuid(db, id).await?;
        let user_data: PermissionData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<PermissionData>, DbErr> {
        debug!("PermissionQuery::search filters: {:?}", filters);
        let result = PermissionQueryManager::filter(db, pagination, order, filters).await;
        let result = match result {
            Ok(res) => res,
            Err(e) => {
                debug!("PermissionQuery::search error: {:?}", e);
                return Err(e);
            }
        };
        debug!("PermissionQuery::search result: {:?}", result);
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        debug!("PermissionQuery::search mapped_result: {:?}", mapped_result);
        Ok(mapped_result)
    }
}
