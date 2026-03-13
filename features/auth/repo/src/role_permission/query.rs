use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::role_permission::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::role_permission::RolePermissionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct RolePermissionQueryManager;

impl RolePermissionQueryManager {
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

pub struct RolePermissionQuery {}

impl RolePermissionQuery {
    pub async fn get<'a>(id: Uuid) -> Result<RolePermissionData, DbErr> {
        let model = RolePermissionQueryManager::get_by_id_uuid(id).await?;
        let user_data: RolePermissionData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<RolePermissionData>, DbErr> {
        debug!("RolePermissionQuery::search filters: {:?}", filters);
        let result = RolePermissionQueryManager::filter(pagination, order, filters).await;
        let result = match result {
            Ok(res) => res,
            Err(e) => {
                debug!("RolePermissionQuery::search error: {:?}", e);
                return Err(e);
            }
        };
        debug!("RolePermissionQuery::search result: {:?}", result);
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        debug!(
            "RolePermissionQuery::search mapped_result: {:?}",
            mapped_result
        );
        Ok(mapped_result)
    }
}
