use dioxus::{logger::tracing::Level, prelude::*};

use views::{Authenticate, Blog, Home};

mod models;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/authenticate?:client_id&:redirect_uri&:response_type&:scope")]
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
const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

fn main() {
    // Init logger
    dioxus::logger::init(Level::INFO).expect("Failed to initialize logger");

    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {

        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { "Hello from my workspace!" }

        Navbar {
            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Home"
            // }
            div { "Navbar here" }

        }


        Router::<Route> {}

    }
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            {children}
        }
    }
}
