use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TranslationAppState {}

impl Default for TranslationAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TranslationCacheState {
    Default
}

impl Default for TranslationCacheState {
    fn default() -> Self {
        Self::Default
    }
}
