use axum::{extract::State, routing::post, Router};

use features_auth_model::{
    authentication::{
        AuthLoginData, AuthLoginDataResponse, AuthLoginRequest, AuthRegisterData,
        AuthRegisterDataResponse, AuthRegisterRequest, AuthenticationCreateRequest,
    },
    login::{LoginCodeRequest, LoginCodeResponse, LoginCodeResponseResponse},
    state::{AuthAppState, AuthCacheState},
};
use features_auth_stream::PRODUCER_KEY;
use shared_shared_app::{doc::ErrorResponse, state::AppState};
use shared_shared_auth::permission::PublicAccess;
use shared_shared_data_app::{
    json::{ResponseJson, ValidJson},
    result::{OkUuid, OkUuidResponse, Result},
};

use features_auth_service::{AuthenticationRequestService, LoginService};

const REQUEST_CODE: &str = "/public/requests/code";
const REQUEST_LOGIN: &str = "/public/requests/login";
const REQUEST_REGISTER: &str = "/public/requests/register";
const LOGIN_CODE: &str = "/public/login/code";

const TAG: &str = "request";

#[utoipa::path(
    post,
    request_body = AuthenticationCreateRequest,
    path = REQUEST_CODE,
    tag = TAG,
    summary = "Request authorization code",
    responses(
        (status = 200, description= "Request success", body= OkUuidResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
async fn request_code(
    _public: PublicAccess,
    ValidJson(request): ValidJson<AuthenticationCreateRequest>,
) -> Result<ResponseJson<OkUuid>> {
    let request_id = AuthenticationRequestService::request(request.into()).await?;
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
    summary = "Login",
    responses(
        (status = 200, description= "Request success", body= AuthLoginDataResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
    )
)]
async fn request_login(
    _public: PublicAccess,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<AuthLoginRequest>,
) -> Result<ResponseJson<AuthLoginData>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let login_data = AuthenticationRequestService::login(&producer, request).await?;
    Ok(ResponseJson(login_data))
}

#[utoipa::path(
    post,
    request_body = AuthRegisterRequest,
    path = REQUEST_REGISTER,
    tag = TAG,
    summary = "Register",
    responses(
        (status = 200, description= "Request success", body= AuthRegisterDataResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
    )
)]
async fn request_register(
    _public: PublicAccess,
    State(state): State<AppState<AuthAppState, AuthCacheState>>,
    ValidJson(request): ValidJson<AuthRegisterRequest>,
) -> Result<ResponseJson<AuthRegisterData>> {
    let producer = state
        .get_producer(PRODUCER_KEY.to_string())
        .expect("Producer not found");
    let register_data = AuthenticationRequestService::register(&producer, request).await?;
    Ok(ResponseJson(register_data))
}

pub fn routes(app_state: &AppState<AuthAppState, AuthCacheState>) -> Router {
    Router::new()
        .route(REQUEST_CODE, post(request_code))
        .route(REQUEST_LOGIN, post(request_login))
        .route(REQUEST_REGISTER, post(request_register))
        .route(LOGIN_CODE, post(verify_login_code))
        .with_state(app_state.clone())
}

#[utoipa::path(
    post,
    request_body = LoginCodeRequest,
    path = LOGIN_CODE,
    tag = TAG,
    summary = "Verify login code",
    responses(
        (status = 200, description = "Verification success", body = LoginCodeResponseResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 403, description = "Code not found or expired", body = ErrorResponse),
    )
)]
pub async fn verify_login_code(
    _public: PublicAccess,
    ValidJson(request): ValidJson<LoginCodeRequest>,
) -> Result<ResponseJson<LoginCodeResponse>> {
    let response = LoginService::verify_login_code(request.user_id, request.login_code).await?;
    Ok(ResponseJson(response))
}
