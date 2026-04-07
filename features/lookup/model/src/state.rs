use serde::{Deserialize, Serialize};

use crate::lookup_item::LookupItemData;

#[derive(Clone, Serialize, Deserialize)]
pub struct LookupAppState {}

impl Default for LookupAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum LookupCacheState {
    Default,
    LookupItems { datas: String },
}

impl Default for LookupCacheState {
    fn default() -> Self {
        Self::Default
    }
}
