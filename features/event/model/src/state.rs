use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EventAppState {}

impl Default for EventAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EventCacheState {
    Default,
}

impl Default for EventCacheState {
    fn default() -> Self {
        Self::Default
    }
}
