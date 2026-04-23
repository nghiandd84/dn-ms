use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult}, query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;

use features_lookup_model::lookup_type::{
    LookupTypeData, LookupTypeForCreateRequest, LookupTypeForUpdateRequest,
};
use features_lookup_repo::{LookupTypeMutation, LookupTypeQuery};

pub struct LookupTypeService {}

impl LookupTypeService {
    pub async fn create_lookup_type(req: LookupTypeForCreateRequest) -> Result<Uuid, AppError> {
        let lookup_type_id = LookupTypeMutation::create_lookup_type(req.into()).await;
        let id = match lookup_type_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating lookup_type: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create lookup_type".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_lookup_types(
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<LookupTypeData>, AppError> {
        LookupTypeQuery::get_lookup_types(pagination, order, filters).await
    }

    pub async fn get_lookup_type_by_id(id: Uuid, query_params: QueryParams) -> Result<LookupTypeData, AppError> {
        LookupTypeQuery::get_lookup_type_by_id(id, query_params.includes()).await
    }

    pub async fn get_lookup_type_by_code(code: &str) -> Result<LookupTypeData, AppError> {
        LookupTypeQuery::get_lookup_type_by_code(code).await
    }

    pub async fn update_lookup_type(
        id: Uuid,
        req: LookupTypeForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = LookupTypeMutation::update_lookup_type(id, req.into()).await;
        match result {
            Ok(updated) => Ok(updated),
            Err(e) => {
                debug!("Error updating lookup_type: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update lookup_type".to_string(),
                ))
            }
        }
    }

    pub async fn delete_lookup_type(id: Uuid) -> Result<bool, AppError> {
        let result = LookupTypeMutation::delete_lookup_type(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                debug!("Error deleting lookup_type: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete lookup_type".to_string(),
                ))
            }
        }
    }
}
