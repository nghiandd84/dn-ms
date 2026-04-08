use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_profiles_entities::user_preference::{
    ModelOptionDto, UserPreferenceForCreateDto, UserPreferenceForUpdateDto,
};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct UserPreferenceForCreateRequest {
    pub profile_id: Uuid,
    #[validate(length(
        min = 2,
        max = 10,
        code = "language",
        message = "the length of language must be between 2 and 10"
    ))]
    pub language: String,
    #[validate(length(
        min = 4,
        max = 20,
        code = "theme",
        message = "the length of theme must be between 4 and 20"
    ))]
    pub theme: String,
    #[serde(default = "default_notifications_enabled")]
    pub notifications_enabled: bool,
}

fn default_notifications_enabled() -> bool {
    true
}

impl Into<UserPreferenceForCreateDto> for UserPreferenceForCreateRequest {
    fn into(self) -> UserPreferenceForCreateDto {
        UserPreferenceForCreateDto {
            profile_id: self.profile_id,
            language: self.language,
            theme: self.theme,
            notifications_enabled: self.notifications_enabled,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct UserPreferenceForUpdateRequest {
    #[validate(length(
        min = 2,
        max = 10,
        code = "language",
        message = "the length of language must be between 2 and 10"
    ))]
    pub language: Option<String>,
    #[validate(length(
        min = 4,
        max = 20,
        code = "theme",
        message = "the length of theme must be between 4 and 20"
    ))]
    pub theme: Option<String>,
    pub notifications_enabled: Option<bool>,
}

impl Into<UserPreferenceForUpdateDto> for UserPreferenceForUpdateRequest {
    fn into(self) -> UserPreferenceForUpdateDto {
        UserPreferenceForUpdateDto {
            language: self.language,
            theme: self.theme,
            notifications_enabled: self.notifications_enabled,
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct UserPreferenceData {
    id: Option<Uuid>,
    profile_id: Option<Uuid>,
    language: Option<String>,
    theme: Option<String>,
    notifications_enabled: Option<bool>,
}

impl Into<UserPreferenceData> for ModelOptionDto {
    fn into(self) -> UserPreferenceData {
        UserPreferenceData {
            id: self.id,
            profile_id: self.profile_id,
            language: self.language,
            theme: self.theme,
            notifications_enabled: self.notifications_enabled,
        }
    }
}
