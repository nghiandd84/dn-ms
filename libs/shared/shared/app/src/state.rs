use sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};
use shared_shared_data_error::auth::AuthError;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::error;

use shared_shared_auth::permission::StatePermission;
use shared_shared_data_cache::cache::Cache;

use crate::event_task::producer::Producer;

/// A single field-level permission entry: which fields a role can access for a given resource+action.
#[derive(Clone, Debug)]
pub struct FieldPermissionEntry {
    /// Resource identifier (e.g., "LOOKUP:TYPE")
    pub resource: String,
    /// Action bitmask: READ=1, UPDATE=4
    pub action: u32,
    /// Allowed field names for this role+resource+action
    pub fields: Vec<String>,
}

pub struct AppState<T, C = ()>
where
    C: Clone + Serialize + DeserializeOwned + Default + Sync,
    T: Clone,
{
    pub write_db: DatabaseConnection,
    pub cache: Cache<String, C>,
    pub state: Option<T>,
    pub producer: Arc<Mutex<HashMap<String, Producer>>>,
    pub permissions_map: Arc<Mutex<HashMap<String, Vec<(String, u32)>>>>,
    /// Field-level permissions: role_name → Vec<FieldPermissionEntry>
    pub field_permissions_map: Arc<Mutex<HashMap<String, Vec<FieldPermissionEntry>>>>,
}

impl<T, C> Clone for AppState<T, C>
where
    C: Clone + Serialize + DeserializeOwned + Default + Sync,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            write_db: self.write_db.clone(),
            cache: self.cache.clone(),
            state: self.state.clone(),
            producer: self.producer.clone(),
            permissions_map: self.permissions_map.clone(),
            field_permissions_map: self.field_permissions_map.clone(),
        }
    }
}

impl<T, C> StatePermission for AppState<T, C>
where
    C: Clone + Serialize + DeserializeOwned + Default + Sync,
    T: Clone,
{
    fn get_permission_map(&self, role_name: String, resource_name: String) -> u32 {
        let permission_map = self.permissions_map.clone();
        let permission = match permission_map.lock() {
            Ok(guard) => guard.get(&role_name).cloned(),
            Err(_err) => None,
        };
        match permission {
            Some(perms) => perms
                .into_iter()
                .filter(|(res, _)| res == &resource_name)
                .fold(0, |acc, (_, p)| acc | p),
            None => 0,
        }
    }

    fn get_field_permissions(&self, role_name: &str, resource: &str, action: u32) -> Vec<String> {
        let map = match self.field_permissions_map.lock() {
            Ok(guard) => guard,
            Err(_) => return vec![],
        };
        map.get(role_name)
            .map(|entries| {
                entries
                    .iter()
                    .filter(|e| e.resource == resource && e.action == action)
                    .flat_map(|e| e.fields.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn has_field_permissions(&self, resource: &str) -> bool {
        let map = match self.field_permissions_map.lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };
        map.values().any(|entries| entries.iter().any(|e| e.resource == resource))
    }

    async fn pull_permission(&self) -> Result<(), AuthError> {
        Ok(())
    }
}

impl<T, C> AppState<T, C>
where
    C: Clone + Serialize + DeserializeOwned + Default + Sync,
    T: Clone,
{
    pub fn new(write_db: &DatabaseConnection, cache: Cache<String, C>, state: Option<T>) -> Self {
        Self {
            write_db: write_db.clone(),
            cache,
            state,
            producer: Arc::new(Mutex::new(HashMap::new())),
            permissions_map: Arc::new(Mutex::new(HashMap::new())),
            field_permissions_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_state(&self) -> Option<&T> {
        self.state.as_ref()
    }

    pub fn set_permission_map(&mut self, role_name: String, permissions: Vec<(String, u32)>) {
        let mut permission_map = match self.permissions_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("Failed to acquire lock on permissions map: {}", poisoned);
                return;
            }
        };
        permission_map.insert(role_name, permissions);
    }

    pub fn set_field_permission_map(
        &mut self,
        role_name: String,
        entries: Vec<FieldPermissionEntry>,
    ) {
        let mut map = match self.field_permissions_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!(
                    "Failed to acquire lock on field permissions map: {}",
                    poisoned
                );
                return;
            }
        };
        map.insert(role_name, entries);
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
