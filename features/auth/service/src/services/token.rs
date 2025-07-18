use features_auth_entities::token::{TokenForCreateDto, TokenForUpdateDto};
use features_auth_model::token::{GrantType, TokenForCreateRequest};
use sea_orm::DbConn;
use shared_shared_auth::{
    claim::UserAccessData,
    data::AuthorizationCodeData,
    token::{
        create_access_token, create_refresh_token, decode_refresh_token, REFRESH_TOKEN_EXPIRATION,
        TOKEN_EXPIRATION, TOKEN_TYPE,
    },
};
use shared_shared_data_app::{error::AppError, result::Result};
use tracing::{debug, error};
use uuid::Uuid;

use crate::{auth_code::AuthCodeQuery, client::ClientQuery, token::TokenMutation, user::UserQuery};

pub struct TokenService;

impl TokenService {
    // Add methods for token-related operations here
    // For example, create_token, delete_token, get_token, etc.
    pub async fn create_authorization_data<'a>(
        db: &'a DbConn,
        token_request: &'a TokenForCreateRequest,
    ) -> Result<AuthorizationCodeData> {
        // Convert TokenForCreateRequest to TokenForCreateDto
        let grant_type = token_request.grant_type.clone().unwrap();
        let client_id = token_request.client_id.unwrap_or_default();
        let code = token_request.code.clone().unwrap_or_default();

        let mut authorization_data = AuthorizationCodeData {
            ..Default::default()
        };

        let client = ClientQuery::get(db, client_id).await;
        if client.is_err() {
            error!("Error fetching client with id : {:?}", client_id);
            return Err(AppError::EntityNotFound {
                entity: "Client".to_string(),
            });
        }
        let client = client.unwrap();
        debug!("Client found: {:?}", client);

        match grant_type {
            GrantType::AuthorizationCode => {
                let auth_code =
                    AuthCodeQuery::get_by_client_id_and_code(db, client_id, code.clone()).await;
                if auth_code.is_err() {
                    error!(
                        "Error fetching auth_code for client_id: {} and code: {}",
                        client_id, code
                    );
                    return Err(AppError::EntityNotFound {
                        entity: "AuthCode".to_string(),
                    });
                }
                let auth_code = auth_code.as_ref().unwrap();
                let scopes = auth_code.scopes.clone().unwrap_or_default();
                let client_secret = client.client_secret.unwrap_or_default();
                let user_id = auth_code.user_id.unwrap();
                let accesses = UserQuery::get_access_data_by_user_id(db, user_id).await?;
                debug!("AuthCode found: {:?}", auth_code);
                let authorization_code_data = create_new_token_authorization_data(
                    db,
                    user_id,
                    client_id,
                    &client_secret,
                    accesses,
                    scopes,
                )
                .await?;

                authorization_data = authorization_code_data;
                debug!("Access token is created successfully");
            }
            GrantType::RefreshToken => {
                debug!("RefreshToken grant type: {:?}", grant_type);
                let old_refresh_token = token_request.code.clone().unwrap_or_default();
                let client_secret = client.client_secret.unwrap_or_default();
                let authorization_code_data =
                    create_refresh_token_authorization_data(db, &old_refresh_token, &client_secret)
                        .await?;
                authorization_data = authorization_code_data;
                debug!("Refresh token is created successfully");
            }
            _ => {
                debug!("No auth_code needed for grant type: {:?}", grant_type);
            }
        }

        Ok(authorization_data)
    }
}

pub async fn create_new_token_authorization_data<'a>(
    db: &'a DbConn,
    user_id: Uuid,
    client_id: Uuid,
    client_secret: &str,
    accesses: Vec<UserAccessData>,
    scopes: Vec<String>,
) -> Result<AuthorizationCodeData> {
    let access_token = create_access_token(user_id, client_secret, accesses)
        .map_err(|error| AppError::Token(error))?;

    let mut dto: TokenForCreateDto = TokenForCreateDto::default();

    dto.user_id = user_id;
    dto.client_id = client_id;
    dto.access_token = access_token.clone();
    dto.scopes = scopes.clone();
    let token_id = TokenMutation::create(db, dto).await?;
    let refresh_token = create_refresh_token(user_id, client_secret, token_id)
        .map_err(|error| AppError::Token(error))?;

    let token_for_update = TokenForUpdateDto {
        access_token: None,
        refresh_token: Some(refresh_token.clone()),
    };
    TokenMutation::update(db, token_id, token_for_update).await?;

    Ok(AuthorizationCodeData {
        access_token,
        token_type: TOKEN_TYPE.to_string(),
        expires_in: TOKEN_EXPIRATION,
        refresh_token: Some(refresh_token),
        refresh_expires_in: Some(REFRESH_TOKEN_EXPIRATION),
        scopes: Some(scopes), // Optional scope can be added if needed
        user_id,
    })
}

pub async fn create_refresh_token_authorization_data<'a>(
    db: &'a DbConn,
    old_refresh_token: &str,
    client_secret: &str,
) -> Result<AuthorizationCodeData> {
    let refresh_data = decode_refresh_token(old_refresh_token, client_secret).unwrap();
    let user_id = refresh_data.user_id;
    let token_id = refresh_data.token_id;
    // let user = UserQuery::get(db, user_id).await?;

    let accesses = UserQuery::get_access_data_by_user_id(db, user_id).await?;
    let access_token = create_access_token(user_id, client_secret, accesses)
        .map_err(|error| AppError::Token(error))?;
    let refresh_token = create_refresh_token(user_id, client_secret, token_id)
        .map_err(|error| AppError::Token(error))?;
    let token_for_update = TokenForUpdateDto {
        access_token: Some(access_token.clone()),
        refresh_token: Some(refresh_token.clone()),
    };
    TokenMutation::update(db, token_id, token_for_update).await?;

    Ok(AuthorizationCodeData {
        access_token,
        token_type: TOKEN_TYPE.to_string(),
        expires_in: TOKEN_EXPIRATION,
        refresh_token: Some(refresh_token),
        refresh_expires_in: Some(REFRESH_TOKEN_EXPIRATION),
        scopes: None,
        user_id: user_id,
    })
}
