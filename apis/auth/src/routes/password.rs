use axum::{extract::State, routing::post, Router};

use features_auth_model::{
    password::{
        ChangePasswordRequest, PasswordResponse, PasswordResponseResponse,
        RequestChangePasswordRequest, RequestPasswordResetRequest, ResetPasswordRequest,
    },
    state::{AuthAppState, AuthCacheState},
};
use features_auth_service::PasswordService;
use features_auth_stream::PRODUCER_KEY;
use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::{Auth, PublicAccess};
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};

use crate::permission::CanUpdateUser;

const REQUEST_CHANGE_PASSWORD: &str = "/passwords/change-request";
const CHANGE_PASSWORD: &str = "/passwords/change";
const REQUEST_RESET: &str = "/public/passwords/reset-request";
const RESET_PASSWORD: &str = "/public/passwords/reset";

const TAG: &str = "password";

#[utoipa::path(
    post,
    request_body = RequestChangePasswordRequest,
    path = REQUEST_CHANGE_PASSWORD,
    tag = TAG,
    summary = "Request a change password code (sent via email)",
    responses(
        (status = 200, description = "Change code sent", body = PasswordResponseResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn request_change_password(
    auth: Auth<CanUpdateUser>,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(_request): ValidJson<RequestChangePasswordRequest>,
) -> Result<ResponseJson<PasswordResponse>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let user_id = auth.user_id;
    let response = PasswordService::request_change_password(&producer, user_id).await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    request_body = ChangePasswordRequest,
    path = CHANGE_PASSWORD,
    tag = TAG,
    summary = "Change password using the change code",
    responses(
        (status = 200, description = "Password changed successfully", body = PasswordResponseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Change code not found", body = ErrorResponse),
        (status = 410, description = "Change code expired", body = ErrorResponse),
    )
)]
async fn change_password(
    auth: Auth<CanUpdateUser>,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<ChangePasswordRequest>,
) -> Result<ResponseJson<PasswordResponse>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let user_id = auth.user_id;
    let response = PasswordService::change_password(
        &producer,
        user_id,
        request.change_code,
        request.new_password,
    )
    .await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    request_body = RequestPasswordResetRequest,
    path = REQUEST_RESET,
    tag = TAG,
    summary = "Request a password reset code (sent via email)",
    responses(
        (status = 200, description = "Reset code sent", body = PasswordResponseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn request_reset(
    _public: PublicAccess,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<RequestPasswordResetRequest>,
) -> Result<ResponseJson<PasswordResponse>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let response = PasswordService::request_reset(&producer, request.email).await?;
    Ok(ResponseJson(response))
}

#[utoipa::path(
    post,
    request_body = ResetPasswordRequest,
    path = RESET_PASSWORD,
    tag = TAG,
    summary = "Reset password using a reset code",
    responses(
        (status = 200, description = "Password reset successfully", body = PasswordResponseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Reset code not found", body = ErrorResponse),
        (status = 410, description = "Reset code expired", body = ErrorResponse),
    )
)]
async fn reset_password(
    _public: PublicAccess,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<ResetPasswordRequest>,
) -> Result<ResponseJson<PasswordResponse>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let response =
        PasswordService::reset_password(&producer, request.email, request.reset_code, request.new_password)
            .await?;
    Ok(ResponseJson(response))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route(REQUEST_CHANGE_PASSWORD, post(request_change_password))
        .route(CHANGE_PASSWORD, post(change_password))
        .route(REQUEST_RESET, post(request_reset))
        .route(RESET_PASSWORD, post(reset_password))
        .with_state(app_state.clone())
}
