use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum EmailTemplateCacheState {
    Default,
}

impl Default for EmailTemplateCacheState {
    fn default() -> Self {
        Self::Default
    }
}
