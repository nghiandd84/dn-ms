use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    error::{AppError, AuthError},
    json::ResponseJson,
    result::Result,
};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(
    State(_state): State<AppState>,
    payload: Json<LoginPayload>,
) -> Result<ResponseJson<SuccessData>> {
    if payload.username == "demo2" {
        return Err(AppError::EntityNotFound {
            entity: "User".to_string()
        });
    }
    if payload.username == "demo3" {
        return Err(AppError::Auth(AuthError::CtxNotInRequestExt));
    }
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(AppError::Auth(AuthError::LoginFail));
    }

    // Set cookies
    let success_data: SuccessData = SuccessData {
        id: 1,
        token: "my_token".to_owned(),
    };

    Ok(ResponseJson(success_data))
}

pub fn routes(app_state: &AppState) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .with_state(app_state.clone())
}

#[derive(Serialize, Clone)]
struct SuccessData {
    id: i32,
    token: String,
}
