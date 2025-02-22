use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use uuid::Uuid;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

use features_auth_entities::user::UserForUpdateProfileDto;
use features_auth_model::profile::UserForUpdateProfileRequest;
use features_auth_service::user::UserMutation;

#[utoipa::path(
    post,
    request_body = UserForUpdateProfileRequest,
    params  (
        ("user_id" = String, Path, description = "User Id"),
    ),
    path = "/profile/{user_id}",
    tag = "profile",
    description = "Change Profile Data",
    responses(
        (status = 200, description= "Profile was updated", body= OkUuidResponse),       
    )
)]
async fn change_profile(
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    ValidJson(profile_request): ValidJson<UserForUpdateProfileRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let dto: UserForUpdateProfileDto = profile_request.into();
    UserMutation::update_profile(&state.conn, user_id, dto).await?;

    Ok(ResponseJson(OkUuid { ok: true, id: None }))
}

pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/profile/{user_id}", post(change_profile))
        .with_state(app_state.clone())
}
