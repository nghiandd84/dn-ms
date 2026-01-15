use sea_orm::DbConn;
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::permission::{PermissionData, PermissionForCreateRequest};
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

    pub async fn get<'a>(db: &'a DbConn, permission_id: Uuid) -> Result<PermissionData> {
        let permission = PermissionQuery::get(db, permission_id).await?;
        Ok(permission.into())
    }

    pub async fn delete<'a>(db: &'a DbConn, permission_id: Uuid) -> Result<bool> {
        let result = PermissionMutation::delete(db, permission_id).await?;
        Ok(result)
    }
}
