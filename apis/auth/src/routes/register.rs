use axum::{extract::State, routing::post, Router};

use shared_shared_app::state::AppState;

use shared_shared_data_app::{
    error::AppError,
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

use features_auth_model::state::AuthCacheState;
use features_auth_model::user::UserForCreateRequest;
use features_auth_service::services::RegisterService;

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
    state: State<AppState<AuthCacheState>>,
    ValidJson(register_request): ValidJson<UserForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let result = RegisterService::register(&state.conn, register_request).await;
    if let Ok(user_id) = result {
        return Ok(ResponseJson(OkUuid {
            ok: true,
            id: Some(user_id),
        }));
    }
    let err = result.err().unwrap();
    Err(AppError::Auth(err))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .with_state(app_state.clone())
}
