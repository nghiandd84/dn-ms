use dioxus::{fullstack::Redirect, prelude::*};
use dioxus_i18n::{prelude::*, t};
use unic_langid::LanguageIdentifier;

use crate::{models::context::Context, routes::Route};

use crate::models::context::Languages;

#[cfg(feature = "server")]
use {crate::models::authenticate::AuthenticateScreen, dioxus::logger::tracing::debug};

use crate::models::authenticate::AuthenticateParams;

// http://127.0.0.1:8080/request?client_id=b9794d29-c2a2-47f5-9ed2-a9821b4a92a7&scope=openid+profile+email+offline_access&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2Fauth_result&response_type=code&state=eyJmaW5nZXJwcmludCI6Ik15UHJpbmdlcnByaW50IiwidGltZXN0YW1wIjoxNzYxODc5MzEwNzU5fQ%3D%3D&screen=login
#[get("/request?{query}")]
async fn authenticatie(query: AuthenticateParams) -> Result<Redirect> {
    debug!("Authentecate with params: {query:?}");
    let state = crate::services::authenticate::create_authenticate_state(query.clone()).await;
    if state.is_ok() {
        let state = state.unwrap();
        if query.screen == AuthenticateScreen::Login {
            debug!("Redirect to login page with state: {state}");
            return Ok(Redirect::permanent(&format!("/login?state={}", state)));
        } else if query.screen == AuthenticateScreen::SignUp {
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

#[component]
pub fn Root() -> Element {
    let request_context = use_context::<Context>();
    debug!("Request context: {:?}", request_context);
    let mut i18n = i18n(); // Access the i18n context
    let language = request_context.accept_language();
    let language_identifier = LanguageIdentifier::from_bytes(language.as_bytes()).unwrap();
    // Set initial language based on request context
    i18n.set_language(language_identifier);

    // Change  language
    let change_to_vietnamese = move |_| {
        debug!("Changing language to Vietnamese");
        set_language_to_cookie(Languages::ViVn);
        let vi_language = LanguageIdentifier::from_bytes(Languages::ViVn.as_bytes()).unwrap();
        i18n.set_language(vi_language); // Dynamic switching
    };

    let change_to_english = move |_| {
        debug!("Changing language to English");
        set_language_to_cookie(Languages::EnUs);
        let en_language = LanguageIdentifier::from_bytes(Languages::EnUs.as_bytes()).unwrap();
        i18n.set_language(en_language);
    };

    rsx! {
        div {
            button { class:"m-5",  onclick: change_to_english, "English" }
            button { class:"m-5",  onclick: change_to_vietnamese, "Tiếng Việt" }
            Router::<Route> {}
        }
    }
}

fn set_language_to_cookie(lan: Languages) {
    let cookie_value = format!("accept_language={}; Path=/; SameSite=Lax", lan.as_str());
    debug!("Setting language cookie: {}", cookie_value);
}

