use dioxus::{html::link, prelude::*};
use dioxus_i18n::t;
use serde::{Deserialize, Serialize, de};

use crate::{models::context::Context, routes::Route, ui::TextWithLink};

#[component]
pub fn Login(state: String) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
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
            };
            let _ = login(formData).await;
        },


        div { class: "screen flex justify-center items-center bg-slate-50",
          div { class: "border-solid border-2 border-slate-100 rounded-lg px-3 py-5 w-1/4",
            div { class: "text-center text-3xl", {t!("login.title")} }
            if !error_msg.to_string().is_empty() {
              div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
                " {error_msg}"
              }
            }
            div { class: "my-5",
              div { class: "text-lg", { t!("login.email")}}
              input {
                class: "w-full rounded-lg px-2 py-1",
                r#type: "text",
                value: username,
                oninput: move |e| username.set(e.value()),
              }
            }
            div { class: "my-5",
              div { class: "text-lg", { t!("login.password")  } }
              input {
                class: "w-full rounded-lg px-2 py-1",
                r#type: "password",
                value: password,
                oninput: move |e| password.set(e.value()),
              }
            }
            button { r#type: "submit", value: "Submit", {t!("login.submit_title")} }


            button {
              class: "bg-sky-500 text-slate-50 px-3 py-2 rounded-lg w-full my-5 hover:bg-sky-600",
              onclick: move |_| async move {
                  debug!("Attempting to log in with username: {} and password: {}", username, password);
                  // match log_in(username(), password()).await {
                  //     Ok(_) => {
                  //         match navigator.push(Route::User {}) {
                  //             Some(_) => {}
                  //             None => {}
                  //         }
                  //     }
                  //     Err(e) => {
                  //         error_msg
                  //             .set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
                  //     }
                  // }
              },
              {t!("login.submit_title")}
          }

            div {
              // {t!("login.dont_have_account")}
              TextWithLink {
                  id: "login.dont_have_account",
                  to: signup_url,
                  class: "not-have-account" // CSS styling
              }
              // Link { to: Route::SignUp {}, class: "text-sky-400", "Register now" }
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
}

// #[post("/api/login")]
#[server]
// Or #[server] for a general RPC endpoint
pub async fn login(data: FormData) -> Result<()> {
    debug!("Received form data on server: {:?}", data);
    // Your server-side processing logic (e.g., database interaction) goes here.
    // If successful, return Ok(()) or some serializable result.
    Ok(())
}
