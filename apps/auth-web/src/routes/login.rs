use dioxus::prelude::*;
use dioxus_i18n::t;
use serde::{Deserialize, Serialize};

use crate::{models::context::Context, routes::Route, ui::TextWithLink};

#[component]
pub fn Login(state: String) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let state_clone = use_signal(|| state.clone());
    let mut error_msg = use_signal(|| String::new());
    let navigator = use_navigator();

    let context = use_context::<Context>();
    debug!("Login component context: {:?}", context);

    let link_to_signup = Route::SignUp {
        state: state.clone(),
    };
    // link_to_signup.to_string();
    debug!("Link to signup route: {:?}", link_to_signup);
    let signup_url = link_to_signup.to_string();
    debug!("Signup URL: {}", signup_url);

    rsx!(
      form {
        id: "login-form",
        onsubmit: move |ev| async move {
            debug!("Form submitted to log in with username: {} and password: {}", username, password);
            ev.prevent_default();
            let formData = FormData {
                email: username.to_string(),
                password: password.to_string(),
                state: state_clone.to_string()
            };
            match login(formData).await {
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
            div { class: "text-center text-3xl", {t!("login.title")} }
            if !error_msg.to_string().is_empty() {
              div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
                " {error_msg}"
              }
            }
            div { class: "my-5",
              label {
                for: "username",
                class: "block mb-2.5 text-sm font-medium text-heading",
                { t!("login.email") }
              }
              input {
                id: "username",
                class: "bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body",
                r#type: "text",
                value: username,
                oninput: move |e| username.set(e.value()),
              }
            }
            div { class: "my-5",
              label {
                for: "password",
                class: "block mb-2.5 text-sm font-medium text-heading",
                { t!("login.password")  }
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
              r#type: "submit", value: "Submit", {t!("login.submit_title")} }

            div {
              TextWithLink {
                  id: "login.dont_have_account",
                  to: signup_url,
                  class: "not-have-account" // CSS styling
              }
            }
          }
        }
    }
       )
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FormData {
    email: String,
    password: String,
    state: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse {
    pub id_token: String,
    pub redirect_uri: String,
}

#[server]
pub async fn login(data: FormData) -> Result<LoginResponse> {
    debug!("Received form data on server: {:?}", data);
    let login_data = features_auth_remote::AuthenticationRequestService::login_password(
        data.email,
        data.password,
        uuid::Uuid::parse_str(&data.state).unwrap_or_default(),
    )
    .await;
    let login_data = match login_data {
        Ok(info) => {
            debug!("Login successful, received data: {:?}", info);
            LoginResponse {
                id_token: info.id_token,
                redirect_uri: info.redirect_uri,
            }
        }
        Err(e) => {
            debug!("Login failed with error: {}", e);
            let err_msg = format!("Login failed");
            return Err(dioxus::CapturedError::from_display(err_msg));
        }
    };
    Ok(login_data)
}
