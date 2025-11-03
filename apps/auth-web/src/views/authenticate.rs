use crate::models::authenticate::AuthenticateParams;
use dioxus::{
    logger::tracing::{debug, info},
    prelude::*,
};
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
    let breed_list = use_loader(move || async move {
        reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await?
            .json::<ListBreeds>()
            .await
    })?;
    // use_effect(|| {
    //     // You can perform side effects here, such as fetching data or initializing state
    //     info!("Call API to validate authentication request...");
    // });
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
}
