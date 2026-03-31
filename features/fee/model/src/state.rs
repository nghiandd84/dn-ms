use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FeeAppState {}

impl Default for FeeAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FeeCacheState {
    Default
}

impl Default for FeeCacheState {
    fn default() -> Self {
        Self::Default
    }
}
