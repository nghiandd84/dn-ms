use crate::models::request::RequestParams;
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn create_authenticate_state(authenticate_params: RequestParams) -> Result<String> {
    debug!("Create state code on the server...");

    let code = features_auth_remote::AuthenticationRequestService::authenticate_request(
        uuid::Uuid::parse_str(&authenticate_params.client_id).unwrap(),
        authenticate_params
            .scope
            .split('+')
            .map(|s| s.to_string())
            .collect(),
        authenticate_params.redirect_uri.clone(),
        authenticate_params.response_type.clone(),
        authenticate_params.state.clone(),
    )
    .await;

    if code.is_err() {
        let err_msg = code.err().unwrap();
        return Err(dioxus::CapturedError::from_display(err_msg));
    }

    let code = code.unwrap();

    Ok(code.to_string())
}

// #[cfg(not(feature = "server"))]
// pub async fn create_authenticate_state(_authenticate_params: AuthenticateParams) -> Result<String> {
//     let err_msg = "Client-side state creation is not implemented yet.";
//     Err(dioxus::CapturedError::from_display(err_msg))
// }
