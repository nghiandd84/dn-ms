use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthAppState {}

impl Default for AuthAppState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AuthCacheState {
    AccessToken(Uuid),
    RefreshToken(Uuid),
}
