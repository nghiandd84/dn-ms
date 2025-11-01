use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum AuthenticateScreen {
    Login,
    SignUp,
}

#[derive(Deserialize, Serialize)]
pub struct AuthenticateParams {
    pub client_id: String,
    pub scope: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: String,
    pub screen: AuthenticateScreen,
}
