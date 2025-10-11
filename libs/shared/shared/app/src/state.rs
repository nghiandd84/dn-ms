use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sea_orm_migration::sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};
use tracing::error;

use shared_shared_data_cache::cache::Cache;

use crate::event_task::producer::Producer;

pub struct AppState<C, T = ()>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub conn: DatabaseConnection,
    pub cache: Cache<String, C>,
    pub state: Option<T>,
    pub producer: Arc<Mutex<HashMap<String, Producer>>>,
}

impl<C, T> Clone for AppState<C, T>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
            cache: self.cache.clone(),
            state: self.state.clone(),
            producer: self.producer.clone(),
        }
    }
}

impl<C, T> AppState<C, T>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub fn new(conn: DatabaseConnection, cache: Cache<String, C>, state: Option<T>) -> Self {
        Self {
            conn,
            cache,
            state,
            producer: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_state(&self) -> Option<&T> {
        self.state.as_ref()
    }

    pub fn get_producer(&self, key: String) -> Option<Producer> {
        let prodecer_map = match self.producer.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("Failed to acquire lock on producer map: {}", poisoned);
                return None;
            }
        };
        prodecer_map.get(&key).cloned()
    }

    pub fn set_producer(&mut self, key: String, producer: Producer) {
        let mut prodecer_map = match self.producer.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("Failed to acquire lock on producer map: {}", poisoned);
                return;
            }
        };
        prodecer_map.insert(key, producer);
    }
}
