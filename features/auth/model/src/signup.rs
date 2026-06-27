use serde::{Deserialize, Serialize};
use shared_shared_macro::Response;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct SignupActiveRequest {
    pub user_id: Uuid,
    #[validate(length(
        min = 1,
        max = 10,
        code = "code",
        message = "the length of code must be between 1 and 10"
    ))]
    pub code: String,
}

#[derive(Serialize, Debug, ToSchema, Response)]
pub struct SignupActiveResponse {
    pub ok: bool,
    pub auth_code: String,
    pub redirect_uri: String,
}
