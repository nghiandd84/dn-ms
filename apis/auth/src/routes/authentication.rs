use axum::{extract::State, routing::post,  Router};

use features_auth_model::{authentication::AuthenticationCreateRequest, state::AuthCacheState};
use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

use features_auth_service::AuthenticationRequestService;

const TAG: &str = "request_authentication";

#[utoipa::path(
    post,
    request_body = AuthenticationCreateRequest,
    path = "/requests",
    tag = TAG,
    responses(
        (status = 200, description= "Request success", body= OkUuidResponse),       
    )
)]
async fn request_authentication(
    State(state): State<AppState<AuthCacheState>>,
    ValidJson(request): ValidJson<AuthenticationCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let request_id = AuthenticationRequestService::request(&state.conn, request.into()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(request_id),
    }))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/requests", post(request_authentication))
        .with_state(app_state.clone())
}
