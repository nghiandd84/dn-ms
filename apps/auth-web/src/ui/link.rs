use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;

#[component]
pub fn TextWithLink(
    /// The key from your .ftl file
    id: String,
    /// Where the link should go (e.g., "/signup")
    to: String,
    /// The class for the <a> tag (optional)
    class: Option<String>,
) -> Element {
    let i18n = i18n();
    // Fetch the raw string: "Do not have an account? <link>Sign up here</link> to get started."
    let raw_msg = i18n.translate(&id);

    // Simple manual parsing to avoid Regex dependencies
    // 1. Split by start tag
    let parts: Vec<&str> = raw_msg.split("<link>").collect();

    if parts.len() < 2 {
        // If no tag found, just return text
        return rsx! { "{raw_msg}" };
    }

    let prefix = parts[0]; // "Do not have an account? "

    // 2. Split the remainder by end tag
    let rest: Vec<&str> = parts[1].split("</link>").collect();
    let link_text = rest[0]; // "Sign up here"
    let suffix = if rest.len() > 1 { rest[1] } else { "" }; // " to get started."

    rsx! {
        span {
            "{prefix}"
            Link {
                to: "{to}",
                class: class.unwrap_or_default(),
                "{link_text}"
            }
            "{suffix}"
        }
    }
}
