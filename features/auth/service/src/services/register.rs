use sea_orm::DbConn;
use shared_shared_data_auth::error::AuthError;
use uuid::Uuid;

use features_auth_entities::user::UserForCreateDto;
use features_auth_model::user::UserForCreateRequest;

use crate::user::UserMutation;

pub struct RegisterService {}

impl RegisterService {
    pub async fn register<'a>(
        db: &'a DbConn,
        register_request: UserForCreateRequest,
    ) -> Result<Uuid, AuthError> {
        // Insert into DB
        let dto: UserForCreateDto = register_request.into();
        let user_id = UserMutation::create_user(db, dto).await;
        // Send notification event

        Ok(user_id.unwrap())
    }
}
