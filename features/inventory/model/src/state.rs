use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct InventoryAppState {}

impl Default for InventoryAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum InventoryCacheState {
    Default
}

impl Default for InventoryCacheState {
    fn default() -> Self {
        Self::Default
    }
}
