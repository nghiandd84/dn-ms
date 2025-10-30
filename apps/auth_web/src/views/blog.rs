use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }

        }
    }
}
