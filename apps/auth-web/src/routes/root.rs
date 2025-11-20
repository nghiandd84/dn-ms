use dioxus::{fullstack::Redirect, prelude::*};

#[cfg(feature = "server")]
use {crate::models::authenticate::AuthenticateScreen, dioxus::logger::tracing::debug};

use crate::models::authenticate::AuthenticateParams;

// http://127.0.0.1:8080/request?client_id=b9794d29-c2a2-47f5-9ed2-a9821b4a92a7&scope=openid+profile+email+offline_access&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2Fauth_result&response_type=code&state=eyJmaW5nZXJwcmludCI6Ik15UHJpbmdlcnByaW50IiwidGltZXN0YW1wIjoxNzYxODc5MzEwNzU5fQ%3D%3D&screen=login
#[get("/request?{query}")]
async fn authenticatie(query: AuthenticateParams) -> Result<Redirect> {
    debug!("Authentecate with params: {query:?}");
    let state = crate::services::authenticate::create_authenticate_state(query.clone()).await;
    if state.is_ok() {
        let state = state.unwrap();
        if query.screen == AuthenticateScreen::Login {
            debug!("Redirect to login page with state: {state}");
            return Ok(Redirect::permanent(&format!("/login?state={}", state)));
        } else if query.screen == AuthenticateScreen::SignUp {
            debug!("Redirect to signup page with state: {state}");
            return Ok(Redirect::permanent(&format!("/signup?state={}", state)));
        }
    } else if state.is_err() {
        return Ok(Redirect::permanent(&format!(
            "/error?message={}",
            state.err().unwrap()
        )));
    }
    Ok(Redirect::permanent("/error?message=Unknown error"))
}
