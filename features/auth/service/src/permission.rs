use std::str::FromStr;

use features_auth_entities::role;
use sea_orm::DbConn;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::permission::{
    PermissionData, PermissionForCreateRequest, PermissionForUpdateRequest,
};
use features_auth_repo::permission::{PermissionMutation, PermissionQuery};

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
        let result = PermissionQuery::search(db, pagination, &order, &filters).await?;
        Ok(result)
    }

    pub async fn delete<'a>(db: &'a DbConn, permission_id: Uuid) -> Result<bool> {
        let result = PermissionMutation::delete(db, permission_id).await?;
        Ok(result)
    }
}
