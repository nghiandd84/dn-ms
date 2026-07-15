pub mod api_key;
pub mod shortened_url;
pub mod url_click;

pub use api_key::{ApiKeyMutation, ApiKeyQuery};
pub use shortened_url::{ShortenedUrlMutation, ShortenedUrlQuery};
pub use url_click::{UrlClickMutation, UrlClickQuery};
