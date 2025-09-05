use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::types::Clients;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NotificationCacheState {}

#[derive(Clone)]
pub struct NotificationState {
    clients: Clients,
    user_to_client: HashMap<Uuid, usize>,
}

impl NotificationState {
    pub fn new(clients: Clients) -> Self {
        Self {
            clients,
            user_to_client: HashMap::new(),
        }
    }

    pub fn get_clients(&self) -> &Clients {
        &self.clients
    }

    pub fn insert_user_client_mapping(&mut self, user_id: Uuid, client_id: usize) {
        self.user_to_client.insert(user_id, client_id);
    }
}
