use dioxus::{fullstack::Redirect, logger::tracing::debug, prelude::*};
use views::{ErrorPage, Home, Login, SignUp};

use crate::models::authenticate::AuthenticateParams;

mod models;
mod services;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
 
    #[route("/login?:state")]
    Login { state: String },
    #[route("/signup?:state")]
    SignUp { state: String },
    
    #[route("/error?:message")]
    ErrorPage { message: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use axum::{
            extract::Request,
            http::StatusCode,
            middleware::{Next, from_fn},
            routing::get,
        };
        use dioxus::server::axum;
        use dotenv::dotenv;

        use features_auth_remote::AuthenticationRequestService;
        use shared_shared_app::discovery::get_consul_client;

        dotenv().ok();

        // dioxus::logger::init(Level::INFO).expect("Failed to initialize logger");

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                debug!("Interval task running...");
                let consul_client = get_consul_client().unwrap();
                AuthenticationRequestService::update_remote(&consul_client).await;
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
        // TODO will remove
        server_router = server_router.layer(from_fn(|request: Request, next: Next| async move {
            // Read the incoming request
            // debug!("Request: {} {}", request.method(), request.uri().path());

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

// http://127.0.0.1:8080/request?client_id=b9794d29-c2a2-47f5-9ed2-a9821b4a92a7&scope=openid+profile+email+offline_access&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2Fauth_result&response_type=code&state=eyJmaW5nZXJwcmludCI6Ik15UHJpbmdlcnByaW50IiwidGltZXN0YW1wIjoxNzYxODc5MzEwNzU5fQ%3D%3D&screen=login
#[get("/request?{query}")]
async fn authenticatie(query: AuthenticateParams) -> Result<Redirect> {
    debug!("Authentecate with params: {query:?}");
    let state = crate::services::authenticate::create_authenticate_state(query.clone()).await;
    if state.is_ok() {
        let state = state.unwrap();
        if query.screen == crate::models::authenticate::AuthenticateScreen::Login {
            debug!("Redirect to login page with state: {state}");
            return Ok(Redirect::permanent(&format!("/login?state={}", state)));
        } else if query.screen == crate::models::authenticate::AuthenticateScreen::SignUp {
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
