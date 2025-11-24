use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequestScreen {
    Login,
    SignUp,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RequestParams {
    pub client_id: String,
    pub scope: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub state: String,
    pub screen: RequestScreen,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub state: String
}

pub struct LoginResponse {
    pub id_token: String,
    pub redirect_uri: String,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub state: String
}
