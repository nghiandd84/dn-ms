use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum BakeryCacheState {
    Default,
}

impl Default for BakeryCacheState {
    fn default() -> Self {
        Self::Default
    }
}
