use sea_orm::DbConn;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::permission::{
    PermissionData, PermissionForCreateRequest, PermissionForUpdateRequest,
};
use features_auth_repo::{
    permission::{PermissionMutation, PermissionQuery},
    role_permission::RolePermissionQuery,
};

pub struct PermissionService {}

impl PermissionService {
    pub async fn create_permission<'a>(
        db: &'a DbConn,
        request: PermissionForCreateRequest,
    ) -> Result<Uuid> {
        let permission_id = PermissionMutation::create(db, request.into()).await?;
        Ok(permission_id)
    }

    pub async fn update_permission<'a>(
        db: &'a DbConn,
        permission_id: Uuid,
        request: PermissionForUpdateRequest,
    ) -> Result<bool> {
        let result = PermissionMutation::update(db, permission_id, request.into()).await?;
        Ok(result)
    }

    pub async fn get<'a>(db: &'a DbConn, permission_id: Uuid) -> Result<PermissionData> {
        let permission = PermissionQuery::get(db, permission_id).await?;
        Ok(permission.into())
    }

    pub async fn search_by_role<'a>(
        db: &'a DbConn,
        role_id: Uuid,
        pagination: &Pagination,
    ) -> Result<QueryResult<PermissionData>> {
        let order = Order::default();
        let role_filter_param = FilterParam::<Uuid> {
            name: "role_id".to_string(),
            value: Some(role_id),
            raw_value: role_id.to_string(),
            operator: FilterOperator::Equal,
        };
        let role_filter = FilterEnum::Uuid(role_filter_param);
        let filters = vec![role_filter];
        let role_permissions =
            RolePermissionQuery::search(db, pagination, &order, &filters).await?;
        let permission_ids: Vec<Uuid> = role_permissions
            .result
            .into_iter()
            .map(|rp| rp.permission_id.unwrap())
            .collect();
        if (permission_ids.is_empty()) {
            return Ok(QueryResult {
                total_page: 0,
                result: vec![],
            });
        }
        let permission_ids_str = permission_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        debug!("Permission IDs: {}", permission_ids_str);
        let permission_filter_param = FilterParam::<Uuid> {
            name: "id".to_string(),
            value: None,
            raw_value: permission_ids_str,
            operator: FilterOperator::In,
        };
        let permission_filter = FilterEnum::Uuid(permission_filter_param);
        let permission_filters = vec![permission_filter];

        let result = PermissionQuery::search(db, pagination, &order, &permission_filters).await?;
        Ok(result)
    }

    pub async fn delete<'a>(db: &'a DbConn, permission_id: Uuid) -> Result<bool> {
        let result = PermissionMutation::delete(db, permission_id).await?;
        Ok(result)
    }
}
