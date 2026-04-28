use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_lookup_model::lookup_item::{
    LookupItemData, LookupItemForCreateRequest, LookupItemForUpdateRequest,
};
use features_lookup_repo::{LookupItemMutation, LookupItemQuery};

use crate::lookup_type::LookupTypeService;

pub struct LookupItemService {}

impl LookupItemService {
    pub async fn create_lookup_item(req: LookupItemForCreateRequest) -> Result<Uuid, AppError> {
        let lookup_item_id = LookupItemMutation::create_lookup_item(req.into()).await;
        let id = match lookup_item_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating lookup_item: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create lookup_item".to_string(),
                ));
            }
        };
        Ok(id)
    }
    pub async fn get_lookup_items_by_type_code(
        tenant_id: &str,
        type_code: &str,
        filters: &FilterCondition,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<LookupItemData>, AppError> {
        let lookup_type = LookupTypeService::get_lookup_type_by_code(tenant_id, type_code).await;
        let lookup_type_id = match lookup_type {
            Ok(lookup_type) => lookup_type.id.unwrap(),
            Err(e) => {
                debug!("Error fetching lookup type by code {}: {:?}", type_code, e);
                return Err(AppError::Internal(
                    "Failed to fetch lookup type".to_string(),
                ));
            }
        };

        LookupItemQuery::get_lookup_items_by_type(lookup_type_id, pagination, order, &filters).await
    }

    pub async fn get_lookup_item_by_id(id: Uuid) -> Result<LookupItemData, AppError> {
        LookupItemQuery::get_lookup_item_by_id(id).await
    }

    pub async fn update_lookup_item(
        id: Uuid,
        req: LookupItemForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = LookupItemMutation::update_lookup_item(id, req.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating lookup_item: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update lookup_item".to_string(),
                ))
            }
        }
    }

    pub async fn delete_lookup_item(id: Uuid) -> Result<bool, AppError> {
        let result = LookupItemMutation::delete_lookup_item(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting lookup_item: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete lookup_item".to_string(),
                ))
            }
        }
    }
}
