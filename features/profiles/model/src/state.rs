use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileAppState {}

impl Default for ProfileAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ProfileCacheState {
    Default,
}

impl Default for ProfileCacheState {
    fn default() -> Self {
        Self::Default
    }
}
