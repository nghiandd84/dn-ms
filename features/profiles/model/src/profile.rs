use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_profiles_entities::profile::{
    ModelOptionDto, ProfileForCreateDto, ProfileForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ProfileForCreateRequest {
    pub user_id: Uuid,
    #[validate(length(
        min = 2,
        max = 100,
        code = "first_name",
        message = "the length of first_name must be between 2 and 100"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 2,
        max = 100,
        code = "last_name",
        message = "the length of last_name must be between 2 and 100"
    ))]
    pub last_name: String,
    #[validate(length(
        min = 0,
        max = 1000,
        code = "bio",
        message = "the length of bio must not exceed 1000 characters"
    ))]
    pub bio: Option<String>,
    #[validate(length(
        min = 0,
        max = 500,
        code = "avatar_url",
        message = "the length of avatar_url must not exceed 500 characters"
    ))]
    pub avatar_url: Option<String>,
    #[validate(length(
        min = 0,
        max = 255,
        code = "location",
        message = "the length of location must not exceed 255 characters"
    ))]
    pub location: Option<String>,
}

impl Into<ProfileForCreateDto> for ProfileForCreateRequest {
    fn into(self) -> ProfileForCreateDto {
        ProfileForCreateDto {
            user_id: self.user_id,
            first_name: self.first_name,
            last_name: self.last_name,
            bio: self.bio.unwrap_or_default(),
            avatar_url: self.avatar_url.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ProfileForUpdateRequest {
    #[validate(length(
        min = 2,
        max = 100,
        code = "first_name",
        message = "the length of first_name must be between 2 and 100"
    ))]
    pub first_name: Option<String>,
    #[validate(length(
        min = 2,
        max = 100,
        code = "last_name",
        message = "the length of last_name must be between 2 and 100"
    ))]
    pub last_name: Option<String>,
    #[validate(length(
        min = 0,
        max = 1000,
        code = "bio",
        message = "the length of bio must not exceed 1000 characters"
    ))]
    pub bio: Option<String>,
    #[validate(length(
        min = 0,
        max = 500,
        code = "avatar_url",
        message = "the length of avatar_url must not exceed 500 characters"
    ))]
    pub avatar_url: Option<String>,
    #[validate(length(
        min = 0,
        max = 255,
        code = "location",
        message = "the length of location must not exceed 255 characters"
    ))]
    pub location: Option<String>,
}

impl Into<ProfileForUpdateDto> for ProfileForUpdateRequest {
    fn into(self) -> ProfileForUpdateDto {
        ProfileForUpdateDto {
            first_name: self.first_name,
            last_name: self.last_name,
            bio: self.bio,
            avatar_url: self.avatar_url,
            location: self.location,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct ProfileData {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
    location: Option<String>,
}

impl Into<ProfileData> for ModelOptionDto {
    fn into(self) -> ProfileData {
        ProfileData {
            id: self.id,
            user_id: self.user_id,
            first_name: self.first_name,
            last_name: self.last_name,
            bio: self.bio,
            avatar_url: self.avatar_url,
            location: self.location,
        }
    }
}
