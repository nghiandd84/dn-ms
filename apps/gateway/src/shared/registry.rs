use async_trait::async_trait;

use crate::error::DakiaResult;

// TODO: create a generic struct for Registry to improve runtime performance, as many places traits are not strictly required
#[async_trait]
pub trait Registry<I> {
    async fn register(&self, key: String, item: I) -> ();
    async fn get(&self, key: &str) -> DakiaResult<Option<I>>;
}
