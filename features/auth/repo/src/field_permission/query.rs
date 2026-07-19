use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_auth_entities::field_permission::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::field_permission::FieldPermissionData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct FieldPermissionQueryManager;

pub struct FieldPermissionQuery {}

impl FieldPermissionQuery {
    pub async fn get(id: Uuid) -> Result<FieldPermissionData, DbErr> {
        let model = FieldPermissionQueryManager::get_by_id_uuid(id).await?;
        let data: FieldPermissionData = model.into();
        Ok(data)
    }

    pub async fn search(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<FieldPermissionData>, DbErr> {
        debug!("FieldPermissionQuery::search filters: {:?}", filters);
        let result = FieldPermissionQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
