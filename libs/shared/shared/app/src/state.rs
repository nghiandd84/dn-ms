use sea_orm_migration::sea_orm::DatabaseConnection;

use serde::{de::DeserializeOwned, Serialize};

use shared_shared_data_cache::cache::Cache;

#[derive(Clone)] 
pub struct AppState<C>
where
    C: Clone + Serialize + DeserializeOwned,
{
    pub conn: DatabaseConnection,
    pub cache: Cache<String, C>,
}

