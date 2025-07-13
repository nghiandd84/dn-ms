use features_auth_entities::token::TokenForCreateDto;
use features_auth_model::token::{GrantType, TokenForCreateRequest};
use sea_orm::DbConn;
use shared_shared_auth::{
    data::AuthorizationCodeData,
    token::{create_access_token, create_authorization_data},
};
use shared_shared_data_app::{error::AppError, result::Result};
use tracing::{debug, error};

use crate::{auth_code::AuthCodeQuery, client::ClientQuery, token::TokenMutation};

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
                debug!("AuthCode found: {:?}", auth_code);
                let authorization_code_data =
                    create_authorization_data(user_id, &client_secret, vec![], scopes)
                        .map_err(|_| AppError::Unknown)?;
                authorization_data = authorization_code_data;
                debug!("Access token created successfully");
            }
            GrantType::RefreshToken => {}
            _ => {
                debug!("No auth_code needed for grant type: {:?}", grant_type);
            } /*
              GrantType::RefreshToken => {
                  token_dto.refresh_token = token_request.code; // Assuming code is used as refresh_token
                  token_dto.client_id = token_request.client_id;
                  token_dto.grant_type = Some(grant_type);
              }
              GrantType::ClientCredentials => {
                  // Handle client credentials grant type if needed
                  token_dto.client_id = token_request.client_id;
                  token_dto.grant_type = Some(grant_type);
              }
               */
        }

        let mut dto: TokenForCreateDto = TokenForCreateDto::default();
        let data = authorization_data.clone();
        dto.user_id = data.user_id;
        dto.client_id = client_id;
        dto.access_token = data.access_token;
        dto.refresh_token = data.refresh_token.unwrap();
        dto.scopes = data.scopes.unwrap_or_default();

        TokenMutation::create(db, dto).await?;

        Ok(authorization_data)
    }
}
