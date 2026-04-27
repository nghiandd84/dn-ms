use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_macro::Query;

use features_auth_entities::permission::Column as PermissionColumn;
use features_auth_entities::permission::Entity as PermissionEntity;
use features_auth_entities::role::{ActiveModel, Column, Entity, ModelOptionDto};
use features_auth_model::role::RoleData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(
    entity(PermissionEntity),
    column(PermissionColumn),
    field(permissions),
    name("permissions")
)]
struct RoleQueryManager;

pub struct RoleQuery {}

impl RoleQuery {
    pub async fn get<'a>(
        id: Uuid,
        query_params: &QueryParams,
        related_filters: &FilterCondition,
    ) -> Result<RoleData, DbErr> {
        let includes = query_params.includes();
        let related_filters_vec = related_filters.collect_leaves();
        let model = RoleQueryManager::get_by_id_uuid_with_related_entities(
            id,
            &includes,
            &related_filters_vec,
        )
        .await?;
        let user_data: RoleData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
        related_filters: &FilterCondition,
    ) -> Result<QueryResult<RoleData>, DbErr> {
        debug!("RoleQuery::search called with pagination: {:?}, order: {:?}, filters: {:?}, query_params: {:?}, related_filters: {:?}", pagination, order, filters, query_params, related_filters);
        let includes = query_params.includes();
        let related_filters_vec = related_filters.collect_leaves();
        let result = RoleQueryManager::filter_with_related_entities(
            pagination,
            order,
            filters,
            &includes,
            &related_filters_vec,
        )
        .await;
        let result = match result {
            Ok(res) => res,
            Err(e) => {
                debug!("RoleQuery::search error: {:?}", e);
                return Err(e);
            }
        };
        debug!("RoleQuery::search result: {:?}", result);
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        debug!("RoleQuery::search mapped_result: {:?}", mapped_result);
        Ok(mapped_result)
    }
}
