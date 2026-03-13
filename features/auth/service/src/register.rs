use tracing::{debug, error};
use uuid::Uuid;

use shared_shared_data_error::auth::AuthError;

use features_auth_entities::user::UserForCreateDto;
use features_auth_model::user::UserForCreateRequest;
use features_auth_repo::{access::AccessMutation, user::UserMutation};

pub struct RegisterService {}

impl RegisterService {
    pub async fn register<'a>(register_request: UserForCreateRequest) -> Result<Uuid, AuthError> {
        let dto: UserForCreateDto = register_request.into();
        let user_id = UserMutation::create_user(dto).await;
        let id = match user_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating user: {:?}", e);
                return Err(AuthError::ExistingUser);
            }
        };
        Ok(id)
    }

    pub async fn assgin_user_to_role<'a>(user_id: Uuid, role_id: Uuid) -> Result<Uuid, AuthError> {
        let result = AccessMutation::create(features_auth_entities::access::AccessForCreateDto {
            user_id,
            role_id,
            key: "".to_string(),
        })
        .await;

        let access_id = match result {
            Ok(id) => id,
            Err(e) => {
                error!("Error assigning role to user: {:?}", e);
                return Err(AuthError::UnknowRole);
            }
        };

        Ok(access_id)
    }
}
