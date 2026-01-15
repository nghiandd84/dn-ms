use sea_orm::DbConn;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::role::{RoleData, RoleForCreateRequest};
use features_auth_repo::{
    role::{RoleMutation, RoleQuery},
    role_permission::{RolePermissionMutation, RolePermissionQuery},
};

pub struct RoleService {}

impl RoleService {
    pub async fn create_role<'a>(db: &'a DbConn, request: RoleForCreateRequest) -> Result<Uuid> {
        let role_id = RoleMutation::create(db, request.into()).await?;
        Ok(role_id)
    }

    pub async fn get<'a>(db: &'a DbConn, role_id: Uuid) -> Result<RoleData> {
        let role = RoleQuery::get(db, role_id).await?;
        Ok(role.into())
    }

    pub async fn delete<'a>(db: &'a DbConn, role_id: Uuid) -> Result<bool> {
        let result = RoleMutation::delete(db, role_id).await?;
        Ok(result)
    }

    pub async fn assign_permissions<'a>(
        db: &'a DbConn,
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<bool> {
        let result =
            RolePermissionMutation::assign_permissions(db, role_id, permission_ids).await?;
        Ok(result)
    }
    pub async fn unassign_permissions<'a>(
        db: &'a DbConn,
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<bool> {
        let param: FilterParam<String> = FilterParam {
            name: "role_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(role_id.clone().to_string()),
            raw_value: role_id.to_string(),
        };
        let email_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![email_filter];
        let pagination = Pagination::new(1, 200);
        let order = Order::default();
        let search = RolePermissionQuery::search(db, &pagination, &order, &filters).await?;
        for dto in search.result {
            if permission_ids.contains(&dto.permission_id.unwrap()) {
                let _ = RolePermissionMutation::delete(db, dto.id.unwrap()).await?;
            }
        }
        Ok(true)
    }
}
