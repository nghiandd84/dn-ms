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

impl RoleQueryManager {
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

pub struct RoleQuery {}

impl RoleQuery {
    pub async fn get<'a>(
        id: Uuid,
        query_params: &QueryParams,
        related_filters: &Vec<FilterEnum>,
    ) -> Result<RoleData, DbErr> {
        let includes = query_params.includes();
        let model =
            RoleQueryManager::get_by_id_uuid_with_related_entities(id, &includes, related_filters)
                .await?;
        let user_data: RoleData = model.into();
        Ok(user_data)
    }

    pub async fn search<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
        query_params: &QueryParams,
        related_filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<RoleData>, DbErr> {
        debug!("RoleQuery::search called with pagination: {:?}, order: {:?}, filters: {:?}, query_params: {:?}, related_filters: {:?}", pagination, order, filters, query_params, related_filters);
        let includes = query_params.includes();
        let result = RoleQueryManager::filter_with_related_entities(
            pagination,
            order,
            filters,
            &includes,
            related_filters,
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
