use sea_orm_migration::sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};

use shared_shared_data_cache::cache::Cache;

pub struct AppState<C, T = ()>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub conn: DatabaseConnection,
    pub cache: Cache<String, C>,
    pub state: Option<T>,
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
        }
    }
}

impl<C, T> AppState<C, T>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub fn new(conn: DatabaseConnection, cache: Cache<String, C>, state: Option<T>) -> Self {
        Self { conn, cache, state }
    }

    pub fn get_state(&self) -> Option<&T> {
        self.state.as_ref()
    }
}
