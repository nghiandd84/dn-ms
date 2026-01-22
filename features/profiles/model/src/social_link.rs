use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_profiles_entities::social_link::{SocialLinkForCreateDto, SocialLinkForUpdateDto, ModelOptionDto};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct SocialLinkForCreateRequest {
    pub profile_id: Uuid,
    #[validate(length(
        min = 2,
        max = 50,
        code = "platform",
        message = "the length of platform must be between 2 and 50"
    ))]
    pub platform: String,
    #[validate(length(
        min = 5,
        max = 500,
        code = "url",
        message = "the length of url must be between 5 and 500"
    ))]
    pub url: String,
}

impl Into<SocialLinkForCreateDto> for SocialLinkForCreateRequest {
    fn into(self) -> SocialLinkForCreateDto {
        SocialLinkForCreateDto {
            profile_id: self.profile_id,
            platform: self.platform,
            url: self.url,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct SocialLinkForUpdateRequest {
    #[validate(length(
        min = 2,
        max = 50,
        code = "platform",
        message = "the length of platform must be between 2 and 50"
    ))]
    pub platform: Option<String>,
    #[validate(length(
        min = 5,
        max = 500,
        code = "url",
        message = "the length of url must be between 5 and 500"
    ))]
    pub url: Option<String>,
}

impl Into<SocialLinkForUpdateDto> for SocialLinkForUpdateRequest {
    fn into(self) -> SocialLinkForUpdateDto {
        SocialLinkForUpdateDto {
            platform: self.platform,
            url: self.url,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct SocialLinkData {
    id: Option<Uuid>,
    profile_id: Option<Uuid>,
    platform: Option<String>,
    url: Option<String>,
}

impl Into<SocialLinkData> for ModelOptionDto {
    fn into(self) -> SocialLinkData {
        SocialLinkData {
            id: self.id,
            profile_id: self.profile_id,
            platform: self.platform,
            url: self.url,
        }
    }
}
