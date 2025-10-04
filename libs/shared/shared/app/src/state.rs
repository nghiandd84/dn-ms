use sea_orm_migration::sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};

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
    pub producer: Option<Producer>,
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
            producer: None,
        }
    }

    pub fn get_state(&self) -> Option<&T> {
        self.state.as_ref()
    }

    pub fn get_producer(&self) -> Option<&Producer> {
        self.producer.as_ref()
    }

    pub fn set_producer(&mut self, producer: Producer) {
        self.producer = Some(producer);
    }
}
