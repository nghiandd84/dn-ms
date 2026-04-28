use axum::{routing::post, Router};

use shared_shared_app::state::AppState;
use shared_shared_auth::permission::PublicAccess;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_error::app::AppError;

use features_auth_model::state::{AuthAppState, AuthCacheState};
use features_auth_model::user::UserForCreateRequest;
use features_auth_service::RegisterService;

#[utoipa::path(
    post,
    request_body = UserForCreateRequest,
    path = "/register",
    tag = "register",
    responses(
        (status = 200, description = "User is created", body = OkUuidResponse),       
    )
)]
async fn register(
    _public: PublicAccess,
    ValidJson(register_request): ValidJson<UserForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let result = RegisterService::register(register_request).await;
    if let Ok(user_id) = result {
        return Ok(ResponseJson(OkUuid {
            ok: true,
            id: Some(user_id),
        }));
    }
    let err = result.err().unwrap();
    Err(AppError::Auth(err))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .with_state(app_state.clone())
}
