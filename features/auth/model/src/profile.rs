use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use features_auth_entities::user::UserForUpdateProfileDto;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct UserForUpdateProfileRequest {
    #[validate(length(
        min = 4,
        max = 16,
        code = "firstname_length",
        message = "the length of first_name must be between 4 and 16"
    ))]
    pub first_name: Option<String>,
    #[validate(length(
        min = 4,
        max = 16,
        code = "lastname_length",
        message = "the length of first_name must be between 4 and 16"
    ))]
    pub last_name: Option<String>,
}

impl Into<UserForUpdateProfileDto> for UserForUpdateProfileRequest {
    fn into(self) -> UserForUpdateProfileDto {
        UserForUpdateProfileDto {
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}
