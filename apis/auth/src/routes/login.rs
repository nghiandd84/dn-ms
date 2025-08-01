use axum::{extract::State, routing::post, Json, Router};
use tracing::debug;

use shared_shared_app::state::AppState;
use shared_shared_data_app::{json::ResponseJson, result::Result};
use features_auth_model::{
    login::{LoginData, LoginDataResponse, LoginRequest},
    state::AuthCacheState,
};
use features_auth_service::LoginService;

#[utoipa::path(
    post,
    request_body = LoginRequest,
    
    path = "/login",
    tag = "login",
    description = "Login to API",
    responses(
        (status = 200, description= "Login success", body= LoginDataResponse),       
    )
)]
async fn login(
    State(state): State<AppState<AuthCacheState>>,
    Json(login_request): Json<LoginRequest>,
) -> Result<ResponseJson<LoginData>> {
    debug!("Login requet  {:?}", login_request);
    let success_data =  LoginService::login(&state.conn, login_request).await;
    let success_data = success_data.unwrap();
    

    // Set cookies
    // let success_data = LoginData {
    //     code: "my_code".to_string(),
    // };

    Ok(ResponseJson(success_data))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .with_state(app_state.clone())
}
