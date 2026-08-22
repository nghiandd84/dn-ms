use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TaggingAppState {}

impl Default for TaggingAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TaggingCacheState {
    Default,
}

impl Default for TaggingCacheState {
    fn default() -> Self {
        Self::Default
    }
}
