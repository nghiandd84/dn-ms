use dioxus::{logger::tracing::debug, prelude::*};

// Not Remove https://github.com/MikeCode00/Dioxus-fullstack-Auth

use crate::models::{context::{Context, get_request_context}};
use crate::routes::Route;

mod models;
mod routes;
mod services;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn App() -> Element {
    debug!("Rendering App component...");
    let context = use_server_future(|| async move {
        let state = get_request_context().await;
        if state.is_err() {
            debug!("Error getting request context: {:?}", state.err());
            return Context::default();
        }
        let state = state.unwrap();
        state
    })?()
    .unwrap_or_default();
    use_context_provider(|| context.clone());
    debug!("Context: {:?}", context);
    // let language = context.accept_language();

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // div {
        //     style: "display:none;",
        //     { language }
        // }
        Router::<Route> {}
    }
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
        use crate::models::state::AppState;

        use features_auth_remote::AuthenticationRequestService;
        use shared_shared_app::discovery::get_consul_client;

        dotenv().ok();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                debug!("Interval task running...");
                let consul_client = get_consul_client().unwrap();
                AuthenticationRequestService::update_remote(&consul_client).await;
            }
        });

        let app_state = std::sync::Arc::new(AppState::new("Auth Web Application"));

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
        server_router =
            server_router.layer(from_fn(|mut request: Request, next: Next| async move {
                let header_map = request.headers();
                

                let accept = header_map
                    .get("accept")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("");
                if !accept.contains("text/html") {
                    // debug!("Accept header does not contain text/html, skipping Context insertion. Accept: {}", accept);
                    return next.run(request).await;
                }

                let accept_language = header_map
                    .get("accept-language")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("No accept-language")
                    .to_string();
                let context = Context::new(accept_language);

                // 2. Insert the data into the request extensions
                request.extensions_mut().insert(context);
                debug!("Inserted Context into request extensions.");

                // Run the handler, returning the response
                let res = next.run(request).await;

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
