use dioxus::prelude::*;

#[component]
pub fn SignUp(state: String) -> Element {
    rsx! {
        div {
            id: "signup",
            h3 { "Signup with state #{state}!" }

        }
    }
}
