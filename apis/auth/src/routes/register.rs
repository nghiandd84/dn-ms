use axum::{extract::State, routing::post, Router};

// use features_auth_stream::PRODUCER_KEY;
use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};
use shared_shared_data_error::app::AppError;

use features_auth_model::state::{AuthAppState, AuthCacheState};
use features_auth_model::user::UserForCreateRequest;
use features_auth_service::RegisterService;
// use tracing::error;

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
    state: State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(register_request): ValidJson<UserForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    // let producer = state.get_producer(PRODUCER_KEY.to_string());
    // if producer.is_none() {
    //     error!("Producer not found");
    //     return Err(AppError::Unknown);
    // }
    // let producer = producer.unwrap();
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

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route("/register", post(register))
        .with_state(app_state.clone())
}
