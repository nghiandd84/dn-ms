use serde::{Deserialize, Serialize};
use dioxus::prelude::ServerFnError;

#[cfg(feature = "server")]
use {
    dioxus::fullstack::{extract, extract::Extension},
    dioxus::logger::tracing::debug,
};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Context {
    accept_language: String,
}

impl Context {
    pub fn new(accept_language: String) -> Self {
        Self { accept_language }
    }
    pub fn accept_language(&self) -> &str {
        &self.accept_language
    }
}

#[cfg(feature = "server")]
pub async fn get_request_context() -> Result<Context, ServerFnError> {
    debug!("Resolving app state from request extensions...");
    let Extension(state) = extract::<Extension<Context>, _>().await?;
    Ok(state)
}
#[cfg(not(feature = "server"))]
pub async fn get_request_context() -> Result<Context, ServerFnError> {
    Ok(Context::default())
}
