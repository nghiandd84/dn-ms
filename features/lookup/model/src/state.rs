use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LookupAppState {}

impl Default for LookupAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum LookupCacheState {
    Default
}

impl Default for LookupCacheState {
    fn default() -> Self {
        Self::Default
    }
}
