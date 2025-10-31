use sea_orm::DbConn;
use uuid::Uuid;

use features_auth_entities::authentication::AuthenticationRequestForCreateDto;
use features_auth_model::authentication::AuthenticationCreateRequest;

use shared_shared_data_app::result::Result;

use features_auth_repo::authentication::AuthenticationRequestMutation;

pub struct AuthenticationRequestService {}

impl AuthenticationRequestService {
    pub async fn request<'a>(db: &'a DbConn, request: AuthenticationRequestForCreateDto) -> Result<Uuid> {
        let request_id = AuthenticationRequestMutation::create(db, request).await;
        Ok(request_id.unwrap())
    }
}
