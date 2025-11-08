use dioxus::prelude::*;

#[component]
pub fn Login(state: String) -> Element {
    rsx! {
        div {
            id: "login",
            h3 { "Login with state #{state}!" }

        }
    }
}
