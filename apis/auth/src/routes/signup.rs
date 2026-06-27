use axum::{routing::post, Router};

use features_auth_model::{
    signup::{SignupActiveRequest, SignupActiveResponse, SignupActiveResponseResponse},
    state::{AuthAppState, AuthCacheState},
};
use features_auth_service::ActiveCodeService;
use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::PublicAccess;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::Result,
};

const SIGNUP_ACTIVE: &str = "/public/signup/active";
const TAG: &str = "signup";

#[utoipa::path(
    post,
    request_body = SignupActiveRequest,
    path = SIGNUP_ACTIVE,
    tag = TAG,
    summary = "Activate user account",
    responses(
        (status = 200, description = "Activation success", body = SignupActiveResponseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Code not found", body = ErrorResponse),
    )
)]
pub async fn activate(
    _public: PublicAccess,
    ValidJson(request): ValidJson<SignupActiveRequest>,
) -> Result<ResponseJson<SignupActiveResponse>> {
    let response = ActiveCodeService::activate(request.user_id, request.code).await?;
    Ok(ResponseJson(response))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route(SIGNUP_ACTIVE, post(activate))
        .with_state(app_state.clone())
}
