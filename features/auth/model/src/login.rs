use serde::{Deserialize, Serialize};
use shared_shared_macro::Response;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct LoginRequest {
    pub client_id: Uuid,
    #[validate(email(
        code = "login_email_invalid",
        message = "email must be a valid email address"
    ))]
    pub email: String,
    #[validate(length(
        min = 10,
        max = 128,
        code = "password_length",
        message = "the length of password must be between 10 and 128"
    ))]
    pub password: String,
    #[validate(length(
        min = 1,
        code = "login_scopes_required",
        message = "scopes must contain at least one item"
    ))]
    pub scopes: Vec<String>,
    #[validate(url(
        code = "login_redirect_uri_invalid",
        message = "redirect_uri must be a valid URL"
    ))]
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, ToSchema, Response)]
pub struct LoginData {
    pub code: String,
}
