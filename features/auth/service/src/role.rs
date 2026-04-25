use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
    query_params::QueryParams,
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::role::{RoleData, RoleForCreateRequest};
use features_auth_repo::{
    role::{RoleMutation, RoleQuery},
    role_permission::{RolePermissionMutation, RolePermissionQuery},
};

pub struct RoleService {}

impl RoleService {
    pub async fn create_role<'a>(request: RoleForCreateRequest) -> Result<Uuid> {
        let role_id = RoleMutation::create(request.into()).await?;
        Ok(role_id)
    }

    pub async fn get<'a>(role_id: Uuid, query_params: &QueryParams) -> Result<RoleData> {
        let role = RoleQuery::get(role_id, query_params).await?;
        Ok(role.into())
    }

    pub async fn delete<'a>(role_id: Uuid) -> Result<bool> {
        let result = RoleMutation::delete(role_id).await?;
        Ok(result)
    }

    pub async fn assign_permissions<'a>(role_id: Uuid, permission_ids: Vec<Uuid>) -> Result<bool> {
        let result = RolePermissionMutation::assign_permissions(role_id, permission_ids).await?;
        Ok(result)
    }
    pub async fn unassign_permissions<'a>(
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<bool> {
        let param: FilterParam<Uuid> = FilterParam {
            name: "role_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(role_id.clone()),
            raw_value: role_id.to_string(),
        };
        let email_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![email_filter];
        let pagination = Pagination::new(1, 200);
        let order = Order::default();
        let search = RolePermissionQuery::search(&pagination, &order, &filters).await?;
        for dto in search.result {
            debug!(
                "Current permission id {:?} and unassign permission {:?}",
                dto.permission_id, permission_ids
            );
            if permission_ids.contains(&dto.permission_id.unwrap()) {
                debug!(
                    "Unassign permission id {:?} from role id {:?}",
                    dto.permission_id, role_id
                );
                let _ = RolePermissionMutation::delete(dto.id.unwrap()).await?;
            }
        }
        Ok(true)
    }
}
