use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub enum AuthCacheState {
    AccessToken(Uuid),
    RefreshToken(Uuid)
}
