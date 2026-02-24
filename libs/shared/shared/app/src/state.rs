use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sea_orm_migration::sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};
use tracing::error;

use shared_shared_auth::permission::StatePermission;
use shared_shared_data_cache::cache::Cache;
use shared_shared_observability::metrics::StateMetrics;

use crate::event_task::producer::Producer;

pub struct AppState<T, C = ()>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub conn: DatabaseConnection,
    pub cache: Cache<String, C>,
    pub state: Option<T>,
    pub producer: Arc<Mutex<HashMap<String, Producer>>>,
    pub permissions_map: Arc<Mutex<HashMap<String, Vec<(String, u32)>>>>,
    pub metrics: StateMetrics,
}

impl<T, C> Clone for AppState<T, C>
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
            permissions_map: self.permissions_map.clone(),
            metrics: self.metrics.clone(),
        }
    }
}

impl<T, C> StatePermission for AppState<T, C>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    fn get_permission_map(&self, role_name: String, resource_name: String) -> u32 {
        let permission_map = self.permissions_map.clone();
        let permission = match permission_map.lock() {
            Ok(guard) => guard.get(&role_name).cloned(),
            Err(_err) => None,
        };
        match permission {
            Some(perms) => {
                let perm = perms
                    .into_iter()
                    .find(|(res, _)| res == &resource_name)
                    .map(|(_, p)| p)
                    .unwrap_or(0);
                perm
            }
            None => 0,
        }
    }
}

impl<T, C> AppState<T, C>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub fn new(service_name: String, conn: DatabaseConnection, cache: Cache<String, C>, state: Option<T>) -> Self {
        Self {
            conn,
            cache,
            state,
            producer: Arc::new(Mutex::new(HashMap::new())),
            permissions_map: Arc::new(Mutex::new(HashMap::new())),
            metrics: StateMetrics::new(service_name),
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
