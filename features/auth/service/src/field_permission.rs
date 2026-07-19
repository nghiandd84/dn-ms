use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_model::field_permission::{
    FieldPermissionData, FieldPermissionForCreateRequest, FieldPermissionForUpdateRequest,
};
use features_auth_repo::field_permission::{FieldPermissionMutation, FieldPermissionQuery};

pub struct FieldPermissionService {}

impl FieldPermissionService {
    pub async fn create(request: FieldPermissionForCreateRequest) -> Result<Uuid> {
        debug!("Creating field permission: {:?}", request);
        let id = FieldPermissionMutation::create(request.into()).await?;
        Ok(id)
    }

    pub async fn update(id: Uuid, request: FieldPermissionForUpdateRequest) -> Result<bool> {
        debug!("Updating field permission {}: {:?}", id, request);
        let result = FieldPermissionMutation::update(id, request.into()).await?;
        Ok(result)
    }

    pub async fn get(id: Uuid) -> Result<FieldPermissionData> {
        let data = FieldPermissionQuery::get(id).await?;
        Ok(data)
    }

    pub async fn search(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<FieldPermissionData>> {
        let result = FieldPermissionQuery::search(pagination, order, filters).await?;
        Ok(result)
    }

    pub async fn delete(id: Uuid) -> Result<bool> {
        let result = FieldPermissionMutation::delete(id).await?;
        Ok(result)
    }
}
