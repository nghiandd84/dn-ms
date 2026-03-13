use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_profiles_entities::user_preference::UserPreferenceForCreateDto;
use features_profiles_model::{UserPreferenceData, UserPreferenceForCreateRequest};
use features_profiles_repo::user_preference::{UserPreferenceMutation, UserPreferenceQuery};

pub struct UserPreferenceService {}

impl UserPreferenceService {
    pub async fn create_user_preference<'a>(
        preference_request: UserPreferenceForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: UserPreferenceForCreateDto = preference_request.into();
        let preference_id = UserPreferenceMutation::create_user_preference(dto).await;
        let id = match preference_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating user preference: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create user preference".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_user_preference_by_id<'a>(
        preference_id: Uuid,
    ) -> Result<UserPreferenceData, AppError> {
        UserPreferenceQuery::get_user_preference_by_id(preference_id).await
    }

    pub async fn get_user_preference_by_profile_id<'a>(
        profile_id: Uuid,
    ) -> Result<UserPreferenceData, AppError> {
        UserPreferenceQuery::get_user_preference_by_profile_id(profile_id).await
    }

    pub async fn get_user_preferences<'a>(
        pagination: Pagination,
        order: Order,
        filters: Vec<FilterEnum>,
    ) -> Result<QueryResult<UserPreferenceData>, AppError> {
        UserPreferenceQuery::get_user_preferences(&pagination, &order, &filters).await
    }

    pub async fn update_user_preference<'a>(
        preference_id: Uuid,
        preference_request: features_profiles_model::UserPreferenceForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto = preference_request.into();
        let result = UserPreferenceMutation::update_user_preference(preference_id, dto).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error updating user preference: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update user preference".to_string(),
                ))
            }
        }
    }

    pub async fn delete_user_preference<'a>(preference_id: Uuid) -> Result<bool, AppError> {
        let result = UserPreferenceMutation::delete_user_preference(preference_id).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error deleting user preference: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete user preference".to_string(),
                ))
            }
        }
    }
}
