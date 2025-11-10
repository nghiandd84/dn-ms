#![allow(non_snake_case)]

use dioxus::{logger::tracing::debug, prelude::*};

use views::{Authenticate, Blog, ErrorPage, Home, Login};

// #[cfg(feature = "server")]
// use {
//     dioxus::fullstack::Lazy,
//     dioxus::fullstack::axum_core as axum,
//     // dioxus::fullstack::routing::Router,
//     axum::Router
//     // futures::lock::Mutex,
//     // sqlx::{Executor, Row},
//     // std::sync::LazyLock,
// };

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

/*

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    debug!("Starting auth-web application in server mode...");
    dioxus::logger::initialize_default();
    let addr = dioxus::cli_config::fullstack_address_or_localhost();

    let app = Router::new().with_app("/", App);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    // dioxus::launch(App);
    // let launcher = dioxus::LaunchBuilder::server();

    // let router = axum::Router::new()
    //     // Server side render the application, serve static assets, and register server functions
    //     .serve_dioxus_application(
    //         dioxus::server::ServeConfig::builder().enable_out_of_order_streaming(),
    //         App,
    //     )
    //     .into_make_service();
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, router).await.unwrap();

    // launcher::launch(App);
    // dioxus::server::launch(App).await;
}

#[cfg(not(feature = "server"))]
fn main() {
    debug!("Starting auth-web application...");
    dioxus::launch(App);
}
*/
fn App() -> Element {
    debug!("Rendering App component...");
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

#[derive(Debug)]
struct AppState {
    title: String,
}

fn main() {
    // Run `serve()` on the server only
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use axum::{
            extract::Request,
            middleware::{Next, from_fn},
            routing::get,
        };
        use dioxus::server::axum;
        use features_auth_remote::TokenService;
        use shared_shared_app::discovery::get_consul_client;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                debug!("Interval task running...");
                let consul_client = get_consul_client().unwrap();
                TokenService::update_remote(&consul_client).await;
            }
        });

        let app_state = std::sync::Arc::new(AppState {
            title: "Auth Web Application".to_string(),
        });

        // Create a new router for our app using the `router` function
        let mut server_router = dioxus::server::router(App);

        // add a simple route
        server_router = server_router.route(
            "/health",
            get(|| async move {
                let shared_state = app_state.clone();
                debug!("Health check accessed. AppState: {:?}", shared_state);
                "OK"
            }),
        );

        // TODO add app_state into router
        // server_router = server_router.with_state(app_state);

        // add another middleware layer (example: simple auth for /admin)
        /*
        server_router = server_router.layer(from_fn(|req: Request, next: Next| async move {
            if req.uri().path().starts_with("/admin") {
                return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized");
            }
            let res = next.run(req).await;
            res
        }));
         */
        server_router = server_router.layer(from_fn(|request: Request, next: Next| async move {
            // Read the incoming request
            debug!("Request: {} {}", request.method(), request.uri().path());

            // Run the handler, returning the response
            let res = next.run(request).await;

            // Read/write the response
            // debug!("Response: {}", res.status());

            res
        }));

        debug!("Rendering App on server...");
        // .. customize the router, adding layers and new routes

        // And then return the router
        Ok(server_router)
    });

    // When not on the server, just run `launch()` like normal
    #[cfg(not(feature = "server"))]
    {
        debug!("Rendering App on web...");
        dioxus::launch(App);
    }
}
