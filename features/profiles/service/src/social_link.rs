use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterCondition,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_profiles_entities::social_link::SocialLinkForCreateDto;
use features_profiles_model::{SocialLinkData, SocialLinkForCreateRequest};
use features_profiles_repo::social_link::{SocialLinkMutation, SocialLinkQuery};

pub struct SocialLinkService {}

impl SocialLinkService {
    pub async fn create_social_link<'a>(
        social_link_request: SocialLinkForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let dto: SocialLinkForCreateDto = social_link_request.into();
        let link_id = SocialLinkMutation::create_social_link(dto).await;
        let id = match link_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating social link: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create social link".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_social_link_by_id<'a>(link_id: Uuid) -> Result<SocialLinkData, AppError> {
        SocialLinkQuery::get_social_link_by_id(link_id).await
    }

    pub async fn get_social_links_by_profile_id<'a>(
        profile_id: Uuid,
    ) -> Result<Vec<SocialLinkData>, AppError> {
        SocialLinkQuery::get_social_links_by_profile_id(profile_id).await
    }

    pub async fn get_social_links<'a>(
        pagination: Pagination,
        order: Order,
        filters: FilterCondition,
    ) -> Result<QueryResult<SocialLinkData>, AppError> {
        SocialLinkQuery::get_social_links(&pagination, &order, &filters).await
    }

    pub async fn update_social_link<'a>(
        link_id: Uuid,
        social_link_request: features_profiles_model::SocialLinkForUpdateRequest,
    ) -> Result<bool, AppError> {
        let dto = social_link_request.into();
        let result = SocialLinkMutation::update_social_link(link_id, dto).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error updating social link: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update social link".to_string(),
                ))
            }
        }
    }

    pub async fn delete_social_link<'a>(link_id: Uuid) -> Result<bool, AppError> {
        let result = SocialLinkMutation::delete_social_link(link_id).await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                debug!("Error deleting social link: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete social link".to_string(),
                ))
            }
        }
    }
}
