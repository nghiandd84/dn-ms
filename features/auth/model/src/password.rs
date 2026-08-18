use serde::{Deserialize, Serialize};
use shared_shared_macro::Response;
use utoipa::ToSchema;
use validator::Validate;

/// Validates that a password is strong:
/// - At least 10 characters
/// - At most 128 characters
/// - Contains at least one uppercase letter
/// - Contains at least one lowercase letter
/// - Contains at least one digit
/// - Contains at least one special character
fn validate_strong_password(password: &str) -> Result<(), validator::ValidationError> {
    if password.len() < 10 {
        return Err(validator::ValidationError::new("password_too_short")
            .with_message("password must be at least 10 characters".into()));
    }
    if password.len() > 128 {
        return Err(validator::ValidationError::new("password_too_long")
            .with_message("password must be at most 128 characters".into()));
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(validator::ValidationError::new("password_missing_uppercase")
            .with_message("password must contain at least one uppercase letter".into()));
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(validator::ValidationError::new("password_missing_lowercase")
            .with_message("password must contain at least one lowercase letter".into()));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(validator::ValidationError::new("password_missing_digit")
            .with_message("password must contain at least one digit".into()));
    }
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(validator::ValidationError::new("password_missing_special")
            .with_message(
                "password must contain at least one special character".into(),
            ));
    }
    Ok(())
}

/// Request to initiate a password change (authenticated user).
/// System will send a change code to the user's email.
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RequestChangePasswordRequest {}

/// Confirm password change using the change code (authenticated user)
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(
        min = 1,
        max = 10,
        code = "change_code_length",
        message = "the length of change_code must be between 1 and 10"
    ))]
    pub change_code: String,
    #[validate(custom(function = "validate_strong_password"))]
    pub new_password: String,
}

/// Request to initiate a password reset (public, by email)
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RequestPasswordResetRequest {
    #[validate(email(
        code = "email_invalid",
        message = "email must be a valid email address"
    ))]
    pub email: String,
}

/// Request to reset password using the reset code (public)
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct ResetPasswordRequest {
    #[validate(length(
        min = 1,
        max = 10,
        code = "reset_code_length",
        message = "the length of reset_code must be between 1 and 10"
    ))]
    pub reset_code: String,
    #[validate(custom(function = "validate_strong_password"))]
    pub new_password: String,
    #[validate(email(
        code = "email_invalid",
        message = "email must be a valid email address"
    ))]
    pub email: String,
}

/// Response for password operations
#[derive(Serialize, Debug, ToSchema, Response)]
pub struct PasswordResponse {
    pub ok: bool,
    pub message: String,
}
