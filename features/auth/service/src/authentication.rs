use features_auth_stream::{signup::SignUpMessage, AuthMessage};
use sea_orm::DbConn;
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::Pagination,
};
use shared_shared_data_error::{app::AppError, auth::AuthError};

use features_auth_entities::{
    active_code::ActiveCodeForCreateDto, authentication::AuthenticationRequestForCreateDto,
};
use features_auth_model::{
    auth_code::AuthCodeForCreateRequest,
    authentication::{AuthLoginData, AuthLoginRequest, AuthRegisterData, AuthRegisterRequest},
    user::UserForCreateRequest,
};
use features_auth_repo::{
    active_code::mutation::ActiveCodeMutation,
    auth_code::{AuthCodeMutation, AuthCodeQuery},
    authentication::{AuthenticationRequestMutation, AuthenticationRequestQuery},
    client::ClientQuery,
    role::RoleQuery,
    user::{UserMutation, UserQuery},
};

use crate::RegisterService;
use rand::{thread_rng, Rng};

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
        producer: &'a Producer,
        request: AuthRegisterRequest,
    ) -> Result<AuthRegisterData> {
        let email = request.email.clone().unwrap();
        let state_id = Uuid::parse_str(&request.state.unwrap()).map_err(|_e| AppError::Unknown)?;
        let request_code_data = AuthenticationRequestQuery::get(db, state_id).await;
        if request_code_data.is_err() {
            return Err(AppError::EntityNotFound {
                entity: "request".to_string(),
            });
        }
        let request_code_data = request_code_data.unwrap();
        let language = request
            .language
            .clone()
            .unwrap_or_else(|| "en-US".to_string());
        let create_user_request = UserForCreateRequest {
            email: request.email.unwrap(),
            password: request.password.unwrap(),
            language: language.clone(),
        };
        debug!("User data for create: {:?}", create_user_request);

        let client_id = request_code_data.client_id.unwrap();
        let client = ClientQuery::get(db, client_id).await;
        if client.is_err() {
            debug!("Client not found for client_id {}", client_id);
            return Err(AppError::EntityNotFound {
                entity: "client".to_string(),
            });
        }
        let client_data = client.unwrap();
        debug!("Client data: {:?}", client_data);

        let client_key = client_data.client_key.clone();
        let filters = vec![
            FilterEnum::Bool(FilterParam {
                name: "is_default".to_string(),
                operator: shared_shared_data_core::filter::FilterOperator::Equal,
                value: Some(true),
                raw_value: "true".to_string(),
            }),
            FilterEnum::Uuid(FilterParam {
                name: "client_id".to_string(),
                operator: shared_shared_data_core::filter::FilterOperator::Equal,
                value: Some(client_id),
                raw_value: client_id.to_string(),
            }),
        ];

        let default_roles =
            RoleQuery::search(db, &Pagination::default(), &Order::default(), &filters).await;
        if default_roles.is_err() {
            let error = default_roles.err().unwrap();
            debug!("Error fetching default roles: {:?}", error);
            return Err(AppError::Unknown);
        }
        let default_roles = default_roles.unwrap();
        debug!(
            "Default roles for client_id {}: {:?}",
            client_id, default_roles
        );
        if default_roles.result.is_empty() {
            debug!("No default roles found for client_id {}", client_id);
            return Err(AppError::Auth(AuthError::UnknowRole));
        }
        let default_role = &default_roles.result[0];
        debug!("Assigning default role: {:?}", default_role);

        let user_id = RegisterService::register(db, create_user_request.into()).await;
        if user_id.is_err() {
            let error = user_id.err().unwrap();
            debug!("Error creating user: {:?}", error);
            return Err(AppError::Auth(AuthError::ExistingUser));
        }
        let user_id = user_id.unwrap();

        // Generate a 6-digit numeric active code
        let active_code: String = thread_rng()
            .sample_iter(&rand::distributions::Uniform::from(0..10))
            .take(6)
            .map(|n| n.to_string())
            .collect();

        debug!("Generated active code: {}", active_code);
        let active_code_dto = ActiveCodeForCreateDto {
            user_id: user_id,
            code: active_code.clone(),
        };
        let active_code_result = ActiveCodeMutation::create(db, active_code_dto).await;
        if active_code_result.is_err() {
            debug!("Error creating active code for email : {:?}", email);
        }

        // Asign default role to user
        let assign_role_result =
            RegisterService::assgin_user_to_role(db, user_id, default_role.get_id().unwrap()).await;
        if assign_role_result.is_err() {
            let error = assign_role_result.err().unwrap();
            debug!("Error assigning role to user: {:?}", error);
            UserMutation::delete_user(db, user_id).await.ok();
            return Err(AppError::Auth(AuthError::UnknowRole));
        }
        let auth_message = AuthMessage::SignUp {
            message: SignUpMessage::Success {
                user_id,
                email: email.clone(),
                app_key: client_key.unwrap_or_default(),
                active_code,
                language_code: language.clone(),
                client_email: client_data.get_email().unwrap_or_default(),
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        producer.send(&message).await.map_err(|e| {
            debug!("Error sending signup message to Kafka: {:?}", e.reason);
            AppError::Unknown
        })?;

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
