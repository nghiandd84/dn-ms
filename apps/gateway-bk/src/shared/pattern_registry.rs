use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::error::DakiaResult;

use super::{pattern_matcher::PatternMatcher, registry::Registry};

pub struct PatternRegistry {
    registry: RwLock<HashMap<String, Arc<dyn PatternMatcher>>>,
}

impl PatternRegistry {
    pub fn build() -> Self {
        Self {
            registry: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Registry<Arc<dyn PatternMatcher>> for PatternRegistry {
    async fn register(&self, key: String, item: Arc<dyn PatternMatcher>) {
        let mut write_guard = self.registry.write().await;
        write_guard.insert(key, item);
    }

    async fn get(&self, key: &str) -> DakiaResult<Option<Arc<dyn PatternMatcher>>> {
        let read_guard = self.registry.read().await;
        let matcher = read_guard.get(key);
        match matcher {
            None => Ok(None),
            Some(matcher) => Ok(Some(matcher.clone())),
        }
    }
}

pub type PatternRegistryType = Arc<dyn Registry<Arc<dyn PatternMatcher>> + Send + Sync>;
