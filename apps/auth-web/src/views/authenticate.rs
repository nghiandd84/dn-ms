use crate::{
    Route,
    models::authenticate::{self, AuthenticateParams},
};
use dioxus::{
    CapturedError,
    logger::tracing::{debug, info},
    prelude::*,
};

// use dioxus::fullstack::routing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// use features_auth_remote::TokenService;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ListBreeds {
    pub message: HashMap<String, Vec<String>>,
    pub status: String,
}

/*
http://172.25.43.223:8080/authenticate?client_id=DhjTpB9YQPY379KyAJI3BteZhNtT43NN&scope=openid+profile+email+offline_access&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2Fauth_result&response_type=code&state=eyJmaW5nZXJwcmludCI6Ik15UHJpbmdlcnByaW50IiwidGltZXN0YW1wIjoxNzYxODc5MzEwNzU5fQ%3D%3D
 */
#[component]
pub fn Authenticate(
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String,
) -> Element {
    info!(
        "Authenticate client_id: {client_id}, redirect_uri: {redirect_uri}, response_type:  {response_type}, scope:  {scope}, state: {state}"
    );
    let navigator = use_navigator();
    let authenticate_params = AuthenticateParams {
        client_id,
        scope,
        redirect_uri,
        response_type,
        state,
        screen: authenticate::AuthenticateScreen::Login,
    };

    /*
    let authentication_state = use_server_future(move || {
        debug!("Creating AuthenticateParams inside use_server_future...");
        create_authenticate_state(authenticate_params.clone())
    })?
    .unwrap();
    */
    let authentication_state = use_server_future(move || {
        debug!("Creating AuthenticateParams inside use_server_future...");
        create_authenticate_state(authenticate_params.clone())
    })?
    .value()
    .unwrap();

    match &authentication_state {
        Ok(state) => {
            info!("Successfully created authenticate state {state}.");

            navigator.push(Route::Login {
                state: state.clone(),
            });
        }
        Err(e) => {
            debug!("Redirect to error page.");
            // TODO use true HTTP redirect with a status code 
            navigator.push(Route::ErrorPage {
                message: e.to_string(),
            });
            // if let Ok(mut resp) = server_context().response_parts_mut() {
            //     resp.status = StatusCode::UNAUTHORIZED;
            // }
            // // Return an error to stop processing the function
            // return Err(ServerFnError::ServerError("Unauthorized".to_string()));
        }
    }

    /*
    let breed_list = use_loader(move || async move {
        info!("Call Dog API to get list of breeds...");
        reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await?
            .json::<ListBreeds>()
            .await
    })?;
    let len = breed_list.read().message.len();
    info!("Number of breeds received: {len}");

    info!("Redirect to login or signup screen");
    rsx! {
        div {
            h1 { "Authenticate Page" }
            for cur_breed in breed_list.read().message.keys().take(20).cloned() {
            button {
                onclick: move |_| {
                    // breed.call(cur_breed.clone());
                },
                "{cur_breed}"
            },
            span {  " " }
            }

        }
    }
    */
    rsx! {
        h1 {  "Unknown state, redirecting..." }

    }
}

async fn create_authenticate_state(_authenticate_params: AuthenticateParams) -> Result<String> {
    info!("Creating AuthenticateParams...");
    Ok("MY_CODE".to_string())
    // Err(CapturedError::msg("Error when create state"))
}
