use sea_orm_migration::sea_orm::DatabaseConnection;

use serde::{de::DeserializeOwned, Serialize};

use shared_shared_data_cache::cache::Cache;

#[derive(Clone)]
pub struct AppState<C, T = ()>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone,
{
    pub conn: DatabaseConnection,
    pub cache: Cache<String, C>,
    pub state: Option<T>,
}

impl<C, T> AppState<C, T>
where
    C: Clone + Serialize + DeserializeOwned,
    T: Clone
{
    pub fn new(conn: DatabaseConnection, cache: Cache<String, C>, state: Option<T>) -> Self {
        Self { conn, cache, state }
    }
}
