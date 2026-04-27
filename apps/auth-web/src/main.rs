use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::prelude::*;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

// Not Remove https://github.com/MikeCode00/Dioxus-fullstack-Auth

use crate::{
    models::context::{Context, Languages, get_request_context},
    routes::Root,
};

mod models;
mod routes;
mod services;
mod ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "server")]
const COUNTER_KEY: &str = "counter";

#[cfg(feature = "server")]
use unic_langid::langid;

#[derive(Default, Deserialize, Serialize, Debug)]
struct Counter(usize);

fn app() -> Element {
    debug!("Rendering App component...");
    let context = use_server_future(|| async move {
        let state = get_request_context().await;
        if state.is_err() {
            debug!("Error getting request context: {:?}", state.err());
            return Context::default();
        }
        state.unwrap()
    })?()
    .unwrap_or_default();
    use_context_provider(|| context.clone());
    debug!("Context: {:?}", context);

    let _i18n = use_init_i18n(|| {
        let en_lang = LanguageIdentifier::from_bytes(Languages::EnUs.as_bytes()).unwrap();
        let vi_lang = LanguageIdentifier::from_bytes(Languages::ViVn.as_bytes()).unwrap();
        I18nConfig::new(en_lang.clone())
            .with_locale((en_lang.clone(), include_str!("../locales/en-US.ftl")))
            .with_locale((vi_lang, include_str!("../locales/vi-VN.ftl")))
            .with_fallback(en_lang)
    });

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Root {}
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
        use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

        use features_auth_remote::AuthenticationRequestService;
        use shared_shared_app::discovery::get_consul_client;

        use crate::models::state::AppState;

        dotenv().ok();
        // TODO add opentelemetry to project
        // let service_key = "AUTH_WEB_APPLICATION".to_string();
        // let (log_provider, trace_provider) = init_otel_log_and_traces(service_key)
        //     .expect("Failed to initialize logging and tracing");

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(time::Duration::seconds(10)));

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
        let mut server_router = dioxus::server::router(app);

        // add a simple route
        server_router = server_router.route(
            "/health",
            get(|session: Session| async move {
                let shared_state = app_state.clone();
                let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
                session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
                debug!(
                    "Health check accessed. AppState: {:?} and counter: {:?}",
                    shared_state, counter
                );
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
                use crate::models::context::extract_language;

                let header_map = request.headers();

                let accept = header_map
                    .get("accept")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("");
                if !accept.contains("text/html") {
                    // debug!("Accept header does not contain text/html, skipping Context insertion. Accept: {}", accept);
                    return next.run(request).await;
                }

                let cookie_header = header_map
                    .get("cookie")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("");

                let session_id = cookie_header.split(';').map(|c| c.trim()).find_map(|c| {
                    if let Some(v) = c.strip_prefix("session-id=") {
                        Some(v.trim_matches('"').to_string())
                    } else {
                        let new_sid = format!(
                            "{}",
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_nanos()
                        );
                        Some(new_sid)
                    }
                });

                let session_id = session_id.unwrap();
                debug!("session-id: {}", session_id);

                let accept_language = header_map
                    .get("accept-language")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("No accept-language")
                    .to_string();
                let context = Context::new(extract_language(&accept_language));

                // 2. Insert the data into the request extensions
                request.extensions_mut().insert(context);
                debug!("Inserted Context into request extensions.");

                // Run the handler, returning the response
                let mut res = next.run(request).await;

                // Append Set-Cookie header to response
                let cookie_value = format!(
                    "session-id=\"{}\"; Path=/; HttpOnly; SameSite=Lax",
                    session_id
                );
                if let Ok(val) = axum::http::HeaderValue::from_str(&cookie_value) {
                    res.headers_mut()
                        .append(axum::http::header::SET_COOKIE, val);
                }

                res
            }));
        server_router = server_router.layer(session_layer);

        debug!("Rendering App on server...");
        // .. customize the router, adding layers and new routes

        // And then return the router
        Ok(server_router)
    });

    // When not on the server, just run `launch()` like normal
    #[cfg(not(feature = "server"))]
    {
        debug!("Rendering App on web...");
        dioxus::launch(app);
    }
}
