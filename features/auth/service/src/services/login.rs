use sea_orm::DbConn;
use tracing::debug;

use features_auth_model::{
    auth_code::AuthCodeForCreateRequest,
    login::{LoginData, LoginRequest},
};

use shared_shared_data_app::result::Result;

use crate::{
    auth_code::{AuthCodeMutation, AuthCodeQuery},
    user::UserQuery,
};

pub struct LoginService {}

impl LoginService {
    pub async fn login<'a>(db: &'a DbConn, request: LoginRequest) -> Result<LoginData> {
        let user_data =
            UserQuery::get_user_by_email_and_password(db, request.email, request.password).await?;

        debug!("User data {:?}", user_data);

        let auth_code_request: AuthCodeForCreateRequest = AuthCodeForCreateRequest {
            client_id: Some(request.client_id),
            redirect_uri: Some(request.redirect_uri),
            scopes: Some(request.scopes),
            user_id: user_data.id,
        };
        let code_id = AuthCodeMutation::create(db, auth_code_request.into()).await?;
        let auth_code = AuthCodeQuery::get(db, code_id).await?;
        let result = LoginData {
            code: auth_code.code.unwrap(),
        };
        Ok(result)
    }
}
