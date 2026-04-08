use dioxus::prelude::ServerFnError;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use {
    dioxus::fullstack::{extract, extract::Extension},
    dioxus::logger::tracing::debug,
};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Context {
    accept_language: Languages,
}

impl Context {
    #[cfg(feature = "server")]
    pub fn new(accept_language: Languages) -> Self {
        Self { accept_language }
    }
    pub fn accept_language(&self) -> &Languages {
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

// TODO duplicate code with libs/shared/shared/data/core/src/language.rs

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub enum Languages {
    #[serde(rename = "en-US")]
    #[default]
    EnUs,
    #[serde(rename = "vi-VN")]
    ViVn,
}

impl Languages {
    pub fn as_str(&self) -> &str {
        match self {
            Languages::EnUs => "en-US",
            Languages::ViVn => "vi-VN",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Languages::EnUs => b"en-US",
            Languages::ViVn => b"vi-VN",
        }
    }
}

#[cfg(feature = "server")]
pub fn extract_language(accept_language: &str) -> Languages {
    if accept_language.to_lowercase().contains("vi") {
        Languages::ViVn
    } else {
        Languages::EnUs
    }
}
