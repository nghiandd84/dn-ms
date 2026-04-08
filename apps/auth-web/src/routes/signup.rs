use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

use crate::{routes::Route, ui::TextWithLink};

#[component]
pub fn SignUp(state: String) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let state_signal = use_signal(|| state.clone());
    let mut error_msg = use_signal(String::new);
    let navigator = use_navigator();
    let link_to_login = Route::SignUp {
        state: state.clone(),
    };

    // link_to_signup.to_string();
    debug!("Link to login route: {:?}", link_to_login);
    let login_url = link_to_login.to_string();
    debug!("Signup URL: {}", login_url);
    rsx! {
        form {
        id: "login-form",
        onsubmit: move |ev| async move {
            debug!("Form submitted to log in with email: {} and password: {}", email, password);
            ev.prevent_default();
            let context = use_context::<crate::models::context::Context>();
            let language_code = context.accept_language().as_str().to_string();
            let formData = FormData {
                email: email.to_string(),
                password: password.to_string(),
                state: state_signal.to_string(),
                language_code
            };
            match signup(formData).await {
                Ok(data) => {
                  debug!("Login successful, {:?}", data);
                  let redirect_uri = data.redirect_uri;
                  let id_token = data.id_token;
                  let redirect_url = format!("{}?id_token={}", redirect_uri, id_token);
                  debug!("Redirecting to URL: {}", redirect_url);
                  navigator.push(redirect_url);
                },
                Err(e) => {
                  debug!("Login failed: {:?}", e);
                  error_msg.set("Login failed".to_string());
                }
            }
        },
        div { class: "flex justify-center items-center bg-slate-50",
          div { class: "border-solid border-2 border-slate-100 px-3 py-5 w-1 min-w-[400px]",
            div { class: "text-center text-3xl", {t!("signup.title")} }
            if !error_msg.to_string().is_empty() {
              div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
                " {error_msg}"
              }
            }
            div { class: "my-5",
              label {
                for: "username",
                class: "block mb-2.5 text-sm font-medium text-heading",
                { t!("signup.email") }
              }
              input {
                id: "username",
                class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                r#type: "text",
                value: email,
                oninput: move |e| email.set(e.value()),
              }
            }
            div { class: "my-5",
              label {
                for: "password",
                class: "block mb-2.5 text-sm font-medium text-heading",
                { t!("signup.password")  }
              }
              input {
                id: "password",
                class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                r#type: "password",
                value: password,
                oninput: move |e| password.set(e.value()),
              }
            }
            button {
              class: "bg-sky-500 text-slate-50 px-3 py-2 rounded-lg w-full my-5 hover:bg-sky-600",
              r#type: "submit", value: "Submit", {t!("signup.submit_title")} }

            div {
              TextWithLink {
                  id: "signup.already_have_account",
                  to: login_url,
                  class: "have-account"
              }
            }
          }
        }
    }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    email: String,
    password: String,
    state: String,
    language_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpResponse {
    pub id_token: String,
    pub redirect_uri: String,
}

#[server]
pub async fn signup(data: FormData) -> Result<SignUpResponse> {
    debug!("Signup called with data: {:?}", data);
    let register_data = features_auth_remote::AuthenticationRequestService::register_password(
        data.email,
        data.password,
        uuid::Uuid::parse_str(&data.state).unwrap_or_default(),
        data.language_code,
    )
    .await;
    let register_data = match register_data {
        Ok(info) => {
            debug!("register successful, received data: {:?}", info);
            SignUpResponse {
                id_token: info.id_token,
                redirect_uri: info.redirect_uri,
            }
        }
        Err(e) => {
            debug!("Register failed with error: {}", e);
            let err_msg = format!("Register failed");
            return Err(dioxus::CapturedError::from_display(err_msg));
        }
    };
    Ok(SignUpResponse {
        id_token: register_data.id_token,
        redirect_uri: register_data.redirect_uri,
    })
}
