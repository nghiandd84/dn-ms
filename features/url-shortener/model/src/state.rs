use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UrlShortenerAppState {}

impl Default for UrlShortenerAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UrlShortenerCacheState {
    Default,
}

impl Default for UrlShortenerCacheState {
    fn default() -> Self {
        Self::Default
    }
}
