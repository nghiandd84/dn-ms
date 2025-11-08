#![allow(non_snake_case)]

use dioxus::{logger::tracing::debug, prelude::*};

use views::{Authenticate, Blog, ErrorPage, Home, Login};

mod models;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/authenticate?:client_id&:redirect_uri&:response_type&:scope&:state")]
    Authenticate {
        client_id: String,
        redirect_uri: String,
        response_type: String,
        scope: String,
        state: String
    },
    
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    Blog { id: i32 },

    
    #[route("/login?:state")]
    Login { state: String },

    
    #[route("/error?:message")]
    ErrorPage { message: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    debug!("Starting auth-web application...");
    dioxus::launch(App);
}
fn App() -> Element {
    debug!("Rendering App component...");
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
