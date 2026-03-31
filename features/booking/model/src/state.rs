use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BookingAppState {}

impl Default for BookingAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BookingCacheState {
    Default,
}

impl Default for BookingCacheState {
    fn default() -> Self {
        Self::Default
    }
}

