use serde::{Deserialize, Serialize};
use shared_shared_macro::Response;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct LoginRequest {
    pub client_id: Uuid,
    #[validate(length(
        min = 6,
        max = 256,
        code = "email_length",
        message = "the length of email must be between 6 and 256"
    ))]
    pub email: String,
    #[validate(length(
        min = 10,
        max = 128,
        code = "password_length",
        message = "the length of email must be between 10 and 128"
    ))]
    pub password: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, ToSchema, Response)]
pub struct LoginData {
    pub code: String,
}
