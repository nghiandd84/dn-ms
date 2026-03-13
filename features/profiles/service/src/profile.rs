use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_profiles_entities::profile::ProfileForCreateDto;
use features_profiles_model::{ProfileData, ProfileForCreateRequest, ProfileForUpdateRequest};
use features_profiles_repo::profile::{ProfileMutation, ProfileQuery};

pub struct ProfileService {}

impl ProfileService {
    pub async fn create_profile<'a>(
        profile_request: ProfileForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: ProfileForCreateDto = profile_request.into();
        let profile_id = ProfileMutation::create_profile(dto).await;
        let id = match profile_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating profile: {:?}", e);
                return Err(AppError::Internal("Failed to create profile".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn get_profile_by_id<'a>(
        db: &'a DbConn,
        profile_id: Uuid,
    ) -> Result<ProfileData, AppError> {
        ProfileQuery::get_profile_by_id(db, profile_id).await
    }

    pub async fn get_profile_by_user_id<'a>(
        db: &'a DbConn,
        user_id: Uuid,
    ) -> Result<ProfileData, AppError> {
        ProfileQuery::get_profile_by_user_id(db, user_id).await
    }

    pub async fn get_profiles<'a>(
        db: &'a DbConn,
        pagination: Pagination,
        order: Order,
        filters: Vec<FilterEnum>,
    ) -> Result<QueryResult<ProfileData>, AppError> {
        ProfileQuery::get_profiles(db, &pagination, &order, &filters).await
    }

    pub async fn update_profile<'a>(
        profile_id: Uuid,
        profile_request: ProfileForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto = profile_request.into();

        let result = ProfileMutation::update_profile(profile_id, dto).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error updating profile: {:?}", e);
                Err(AppError::Internal("Failed to update profile".to_string()))
            }
        }
    }

    pub async fn delete_profile<'a>(profile_id: Uuid) -> Result<bool, AppError> {
        let result = ProfileMutation::delete_profile(profile_id).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error deleting profile: {:?}", e);
                Err(AppError::Internal("Failed to delete profile".to_string()))
            }
        }
    }
}
