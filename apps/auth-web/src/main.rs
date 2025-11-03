#![allow(non_snake_case)]

use dioxus::prelude::*;

use views::{Authenticate, Blog, Home};

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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    info!("Starting auth-web application...");
    dioxus::launch(App);
}
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
