use sea_orm::TransactionTrait;
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_config::db::DB_WRITE;
use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterParam},
    order::Order,
    paging::Pagination,
    query_params::QueryParams,
};
use shared_shared_data_error::{app::AppError, auth::AuthError};

use features_auth_entities::{
    access::AccessForCreateDto, active_code::ActiveCodeForCreateDto,
    authentication::AuthenticationRequestForCreateDto,
};
use features_auth_model::{
    auth_code::AuthCodeForCreateRequest,
    authentication::{AuthLoginData, AuthLoginRequest, AuthRegisterData, AuthRegisterRequest},
    user::UserForCreateRequest,
};
use features_auth_repo::{
    access::AccessMutation,
    active_code::mutation::ActiveCodeMutation,
    auth_code::AuthCodeMutation,
    authentication::{AuthenticationRequestMutation, AuthenticationRequestQuery},
    client::ClientQuery,
    role::RoleQuery,
    user::{UserMutation, UserQuery},
};
use features_auth_stream::{signin::SignInMessage, signup::SignUpMessage, AuthMessage};

use rand::{thread_rng, Rng};

pub struct AuthenticationRequestService {}

impl AuthenticationRequestService {
    pub async fn request<'a>(request: AuthenticationRequestForCreateDto) -> Result<Uuid> {
        let request_id = AuthenticationRequestMutation::create(request).await;
        Ok(request_id.unwrap())
    }

    pub async fn login<'a>(
        producer: &'a Producer,
        request: AuthLoginRequest,
    ) -> Result<AuthLoginData> {
        // Get request data from request.state\
        let state_id = Uuid::parse_str(&request.state.unwrap()).map_err(|_| AppError::Unknown)?;
        let request_code_data = AuthenticationRequestQuery::get(state_id).await;
        if request_code_data.is_err() {
            return Err(AppError::EntityNotFound {
                entity: "request".to_string(),
            });
        }
        // Validate email and password
        let request_code_data = request_code_data.unwrap();
        let email = request.email.clone().unwrap();
        let user_data = UserQuery::get_user_by_email_and_password(
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
        let user_id = user_data.id.unwrap();

        // Create auth_code from authentication request data
        let auth_code_request = AuthCodeForCreateRequest {
            client_id: Some(request_code_data.client_id.unwrap()),
            redirect_uri: Some(request_code_data.redirect_uri.unwrap()),
            scopes: Some(request_code_data.scopes.unwrap()),
            user_id: user_data.id,
        };
        AuthCodeMutation::create(auth_code_request.into())
            .await
            .map_err(|_| AppError::Unknown)?;

        // Generate login code (6 digits)
        let login_code: String = thread_rng()
            .sample_iter(&rand::distributions::Uniform::from(0..10))
            .take(6)
            .map(|n| n.to_string())
            .collect();

        // Store login code as active_code
        let active_code_dto = ActiveCodeForCreateDto {
            user_id,
            code: login_code.clone(),
        };
        ActiveCodeMutation::create(active_code_dto)
            .await
            .map_err(|_| AppError::Unknown)?;

        // Send Kafka event with login code
        let auth_message = AuthMessage::SignIn {
            message: SignInMessage::LoginCode {
                user_id: user_id.to_string(),
                email,
                login_code,
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending login code message to Kafka: {:?}", e.reason);
        }

        Ok(AuthLoginData { user_id })
    }

    pub async fn register<'a>(
        producer: &'a Producer,
        request: AuthRegisterRequest,
    ) -> Result<AuthRegisterData> {
        let email = request.email.clone().unwrap();
        let state_id = Uuid::parse_str(&request.state.unwrap()).map_err(|_e| AppError::Unknown)?;
        let request_code_data = AuthenticationRequestQuery::get(state_id).await;
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
        let client = ClientQuery::get(client_id).await;
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

        let default_roles = RoleQuery::search(
            &Pagination::default(),
            &Order::default(),
            &FilterCondition::from(filters),
            &QueryParams::default(),
            &FilterCondition::and(vec![]),
        )
        .await;
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

        // Begin transaction for all DB writes
        let db = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let txn = db.begin().await.map_err(|_| AppError::Unknown)?;

        // 1. Create user
        let user_id = UserMutation::create_user_with_txn(create_user_request.into(), &txn)
            .await
            .map_err(|e| {
                debug!("Error creating user: {:?}", e);
                AppError::Auth(AuthError::ExistingUser)
            })?;

        // 2. Generate and save active code
        let active_code: String = thread_rng()
            .sample_iter(&rand::distributions::Uniform::from(0..10))
            .take(6)
            .map(|n| n.to_string())
            .collect();
        debug!("Generated active code: {}", active_code);

        let active_code_dto = ActiveCodeForCreateDto {
            user_id,
            code: active_code.clone(),
        };
        ActiveCodeMutation::create_with_txn(active_code_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error creating active code: {:?}", e);
                AppError::Unknown
            })?;

        // 3. Assign default role to user
        let access_dto = AccessForCreateDto {
            user_id,
            role_id: default_role.get_id().unwrap(),
            key: "".to_string(),
        };
        AccessMutation::create_with_txn(access_dto, &txn)
            .await
            .map_err(|e| {
                debug!("Error assigning role to user: {:?}", e);
                AppError::Auth(AuthError::UnknowRole)
            })?;

        // 4. Create auth code
        let redirect_uri = request_code_data.redirect_uri.clone().unwrap_or_default();
        let auth_code_request: AuthCodeForCreateRequest = AuthCodeForCreateRequest {
            client_id: Some(request_code_data.client_id.unwrap()),
            redirect_uri: Some(request_code_data.redirect_uri.unwrap()),
            scopes: Some(request_code_data.scopes.unwrap()),
            user_id: Some(user_id),
        };
        let (_code_id, auth_code) =
            AuthCodeMutation::create_with_txn(auth_code_request.into(), &txn)
                .await
                .map_err(|e| {
                    debug!("Error creating auth code: {:?}", e);
                    AppError::Unknown
                })?;

        // 5. Commit transaction - all DB writes succeed or none do
        txn.commit().await.map_err(|e| {
            debug!("Error committing transaction: {:?}", e);
            AppError::Unknown
        })?;

        // 6. Send Kafka AFTER commit - DB is consistent regardless of Kafka outcome
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
        if let Err(e) = producer.send(&message).await {
            // Log but don't fail - user is registered, they can request a new code
            debug!("Error sending signup message to Kafka: {:?}", e.reason);
        }

        let result = AuthRegisterData {
            user_id,
            id_token: auth_code,
            redirect_uri,
        };
        Ok(result)
    }
}
