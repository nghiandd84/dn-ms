use sea_orm::DbConn;
use shared_shared_data_error::{app::AppError, auth::AuthError};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_app::result::Result;

use features_auth_entities::{
    authentication::AuthenticationRequestForCreateDto, user::UserForCreateDto,
};
use features_auth_model::{
    auth_code::AuthCodeForCreateRequest,
    authentication::{AuthLoginData, AuthLoginRequest, AuthRegisterData, AuthRegisterRequest},
};
use features_auth_repo::{
    auth_code::{AuthCodeMutation, AuthCodeQuery},
    authentication::{AuthenticationRequestMutation, AuthenticationRequestQuery},
    user::{UserMutation, UserQuery},
};

pub struct AuthenticationRequestService {}

impl AuthenticationRequestService {
    pub async fn request<'a>(
        db: &'a DbConn,
        request: AuthenticationRequestForCreateDto,
    ) -> Result<Uuid> {
        let request_id = AuthenticationRequestMutation::create(db, request).await;
        Ok(request_id.unwrap())
    }

    pub async fn login<'a>(db: &'a DbConn, request: AuthLoginRequest) -> Result<AuthLoginData> {
        // Get request data from request.state\
        let state_id = Uuid::parse_str(&request.state.unwrap()).map_err(|e| AppError::Unknown)?;
        let request_code_data = AuthenticationRequestQuery::get(db, state_id).await;
        if request_code_data.is_err() {
            // return Err("Invalid state".to_string());
            return Err(AppError::EntityNotFound {
                entity: "request".to_string(),
            });
        }
        // Validate email and password
        let request_code_data = request_code_data.unwrap();
        let user_data = UserQuery::get_user_by_email_and_password(
            db,
            request.email.unwrap(),
            request.password.unwrap(),
        )
        .await;
        if user_data.is_err() {
            return Err(AppError::EntityNotFound {
                entity: "user".to_string(),
            });
        }
        let user_data = user_data.unwrap();
        let redirect_uri = request_code_data.redirect_uri.clone().unwrap_or_default();
        let auth_code_request: AuthCodeForCreateRequest = AuthCodeForCreateRequest {
            client_id: Some(request_code_data.client_id.unwrap()),
            redirect_uri: Some(request_code_data.redirect_uri.unwrap()),
            scopes: Some(request_code_data.scopes.unwrap()),
            user_id: user_data.id,
        };
        let code_id = AuthCodeMutation::create(db, auth_code_request.into()).await?;
        let auth_code = AuthCodeQuery::get(db, code_id).await?;
        let result = AuthLoginData {
            id_token: auth_code.code.unwrap(),
            redirect_uri: redirect_uri,
        };
        Ok(result)
    }

    pub async fn register<'a>(
        db: &'a DbConn,
        request: AuthRegisterRequest,
    ) -> Result<AuthRegisterData> {
        let state_id = Uuid::parse_str(&request.state.unwrap()).map_err(|e| AppError::Unknown)?;
        let request_code_data = AuthenticationRequestQuery::get(db, state_id).await;
        if request_code_data.is_err() {
            return Err(AppError::EntityNotFound {
                entity: "request".to_string(),
            });
        }
        let request_code_data = request_code_data.unwrap();
        let user_dto = UserForCreateDto {
            email: request.email.unwrap(),
            password: request.password.unwrap(),
            ..Default::default()
        };

        debug!("Request data for code: {:?}", request_code_data);
        debug!("User data for create: {:?}", user_dto);

        let user_id = UserMutation::create_user(db, user_dto).await;
        if user_id.is_err() {
            let error = user_id.err().unwrap();
            debug!("Error creating user: {:?}", error);
            return Err(AppError::Auth(AuthError::ExistingUser));
        }
        let user_id = user_id.unwrap();
        // TODO: 
        // 1. Get default roles for the client_id
        // 2. Assign roles to the user
        let redirect_uri = request_code_data.redirect_uri.clone().unwrap_or_default();
        let auth_code_request: AuthCodeForCreateRequest = AuthCodeForCreateRequest {
            client_id: Some(request_code_data.client_id.unwrap()),
            redirect_uri: Some(request_code_data.redirect_uri.unwrap()),
            scopes: Some(request_code_data.scopes.unwrap()),
            user_id: Some(user_id),
        };
        let code_id = AuthCodeMutation::create(db, auth_code_request.into()).await?;
        let auth_code = AuthCodeQuery::get(db, code_id).await?;
        let result = AuthRegisterData {
            id_token: auth_code.code.unwrap(),
            redirect_uri: redirect_uri,
        };
        Ok(result)
    }
}
