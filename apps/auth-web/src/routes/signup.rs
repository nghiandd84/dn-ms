use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

use crate::{models::context::Context, routes::Route, ui::TextWithLink};

#[component]
pub fn SignUp(state: String) -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let state_signal = use_signal(|| state.clone());
    let mut error_msg = use_signal(|| String::new());
    let navigator = use_navigator();
    let link_to_login = Route::SignUp {
        state: state.clone(),
    };
    // link_to_signup.to_string();
    debug!("Link to login route: {:?}", link_to_login);
    let login_url = link_to_login.to_string();
    debug!("Signup URL: {}", login_url);
    rsx! {
        // div {
        //     id: "signup",
        //     h3 { "Signup with state #{state}!" }

        // }
        form {
        id: "login-form",
        onsubmit: move |ev| async move {
            debug!("Form submitted to log in with email: {} and password: {}", email, password);
            ev.prevent_default();
            let formData = FormData {
                email: email.to_string(),
                password: password.to_string(),
                state: state_signal.to_string()
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
                  error_msg.set(format!("Login failed"));
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
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpResponse {
    pub id_token: String,
    pub redirect_uri: String,
}

#[server]
pub async fn signup(data: FormData) -> Result<SignUpResponse> {
    debug!("Signup called with data: {:?}", data);
    // Here you would typically call your backend service to handle signup
    // For demonstration, we will just return a dummy response
    Ok(SignUpResponse {
        id_token: "dummy_id_token".to_string(),
        redirect_uri: "https://example.com/welcome".to_string(),
    })
}
