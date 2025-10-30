use dioxus::{
    logger::tracing::{debug, info},
    prelude::*,
};
use dioxus_router::prelude::*;
use serde::Deserialize;

use crate::models::authenticate::AuthenticateParams;
// use features_auth_remote::TokenService;

/*
http://172.25.43.223:8080/authenticate?client_id=DhjTpB9YQPY379KyAJI3BteZhNtT43NN&scope=openid+profile+email+offline_access&redirect_uri=http%3A%2F%2Flocalhost&response_type=code&state=xyzABC123
 */
#[component]
pub fn Authenticate(
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String
) -> Element {
    info!("Authenticate component mounted {client_id}, {redirect_uri}, {response_type}, {scope}");
    use_effect(|| {
        // You can perform side effects here, such as fetching data or initializing state
        info!("Call API to validate authentication request...");
    });
    info!("Redirect to login or signup screen");
    // let query_params = use_route().query::<AuthenticateParams>().unwrap_or_default();
    rsx! {
        div {
            h1 { "Authenticate Page" }
            p { "This is where users can authenticate." }
        }
    }
}
