use sea_orm::DbConn;
use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_error::auth::AuthError;

use features_auth_entities::user::UserForCreateDto;
use features_auth_model::user::UserForCreateRequest;
use features_auth_repo::{access::AccessMutation, user::UserMutation};

pub struct RegisterService {}

impl RegisterService {
    pub async fn register<'a>(
        db: &'a DbConn,
        register_request: UserForCreateRequest
    ) -> Result<Uuid, AuthError> {
        let dto: UserForCreateDto = register_request.into();
        let user_id = UserMutation::create_user(db, dto).await;
        let id = match user_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating user: {:?}", e);
                return Err(AuthError::ExistingUser);
            }
        };
        let message = ProducerMessage {
            payload: serde_json::to_string(&id).unwrap(),
            key: None,
        };
        // producer.send(message)
        //     .send_message(
        //         "auth_user_registered",
        //         serde_json::to_string(&id).unwrap(),
        //     )
        //     .await
        //     .map_err(|e| {
        //         debug!("Error sending message to Kafka: {:?}", e);
        //         AuthError::UnknownProducer
        //     })?;

        Ok(id)
    }

    pub async fn assgin_user_to_role<'a>(
        db: &'a DbConn,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<Uuid, AuthError> {
        let result = AccessMutation::create(
            db,
            features_auth_entities::access::AccessForCreateDto {
                user_id,
                role_id,
                key: "".to_string(),
            },
        )
        .await;

        let access_id = match result {
            Ok(id) => id,
            Err(e) => {
                return Err(AuthError::UnknowRole);
            }
        };

        Ok(access_id)
    }
}
