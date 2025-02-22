use axum::{extract::State, routing::post, Router};

use features_auth_entities::user::UserForCreateDto;
use features_auth_model::user::UserForCreateRequest;
use features_auth_service::user::UserMutation;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

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
    state: State<AppState>,
    ValidJson(register_request): ValidJson<UserForCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: UserForCreateDto = register_request.into();
    let user_id = UserMutation::create_user(&state.conn, dto).await?;

    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(user_id),
    }))
}

pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .with_state(app_state.clone())
}
