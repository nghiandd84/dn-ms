use axum::{extract::State, routing::post, Router};

use features_auth_model::{
    authentication::{
        AuthLoginData, AuthLoginDataResponse, AuthLoginRequest, AuthRegisterData,
        AuthRegisterDataResponse, AuthRegisterRequest, AuthenticationCreateRequest,
    },
    state::AuthCacheState,
};
use features_auth_stream::PRODUCER_KEY;
use shared_shared_app::state::AppState;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

use features_auth_service::AuthenticationRequestService;

const REQUEST_CODE: &str = "/requests/code";
const REQUEST_LOGIN: &str = "/requests/login";
const REQUEST_REGISTER: &str = "/requests/register";

const TAG: &str = "request";

#[utoipa::path(
    post,
    request_body = AuthenticationCreateRequest,
    path = REQUEST_CODE,
    tag = TAG,
    responses(
        (status = 200, description= "Request success", body= OkUuidResponse),       
    )
)]
async fn request_code(
    State(state): State<AppState<AuthCacheState>>,
    ValidJson(request): ValidJson<AuthenticationCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let request_id = AuthenticationRequestService::request(&state.conn, request.into()).await?;
    Ok(ResponseJson(OkUuid {
        ok: true,
        id: Some(request_id),
    }))
}

#[utoipa::path(
    post,
    request_body = AuthLoginRequest,
    path = REQUEST_LOGIN,
    tag = TAG,
    responses(
        (status = 200, description= "Request success", body= AuthLoginDataResponse),       
    )
)]
async fn request_login(
    State(state): State<AppState<AuthCacheState>>,
    ValidJson(request): ValidJson<AuthLoginRequest>,
) -> Result<ResponseJson<AuthLoginData>> {
    let login_data = AuthenticationRequestService::login(&state.conn, request).await?;
    Ok(ResponseJson(login_data))
}

#[utoipa::path(
    post,
    request_body = AuthRegisterRequest,
    path = REQUEST_REGISTER,
    tag = TAG,
    responses(
        (status = 200, description= "Request success", body= AuthRegisterDataResponse),       
    )
)]
async fn request_register(
    State(state): State<AppState<AuthCacheState>>,
    ValidJson(request): ValidJson<AuthRegisterRequest>,
) -> Result<ResponseJson<AuthRegisterData>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let register_data =
        AuthenticationRequestService::register(&state.conn, &producer, request).await?;
    Ok(ResponseJson(register_data))
}

pub fn routes(app_state: &AppState<AuthCacheState>) -> Router {
    Router::new()
        .route(REQUEST_CODE, post(request_code))
        .route(REQUEST_LOGIN, post(request_login))
        .route(REQUEST_REGISTER, post(request_register))
        .with_state(app_state.clone())
}
