use dioxus::prelude::*;

#[component]
pub fn ErrorPage(message: String) -> Element {
    rsx! {
        div {
            id: "error",
            h3 { "Error message #{message}!" }
        }
    }
}
