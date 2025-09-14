use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use crate::types::{ClientSender, Clients};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NotificationCacheState {}

#[derive(Clone)]
pub struct NotificationState {
    clients: Clients,
    user_to_client: HashMap<Uuid, usize>,
}

unsafe impl Send for NotificationState {}

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

    pub fn insert_client(&mut self, client_id: usize, sender: ClientSender) {
        self.clients.insert(client_id, sender);
    }

    pub fn remove_client(&mut self, client_id: &usize) {
        self.clients.remove(client_id);
    }

    pub fn insert_user_client_mapping(&mut self, user_id: Uuid, client_id: usize) {
        self.user_to_client.insert(user_id, client_id);
    }

    pub fn get_client_id_by_user(&self, user_id: &Uuid) -> Option<usize> {
        self.user_to_client.get(user_id).copied()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{new_clients, Clients};

    #[tokio::test] // Changed to tokio::test since we're using async types
    async fn test_new_notification_state() {
        let clients: Clients = new_clients();
        let state = NotificationState::new(clients.clone());
        let state_client = state.get_clients();
        assert!(state_client.keys().eq(state.clients.keys()));
        assert!(state.user_to_client.is_empty());
    }

    #[tokio::test]
    async fn test_insert_user_client_mapping() {
        let clients: Clients = new_clients();
        let mut state = NotificationState::new(clients);
        let user_id = uuid::Uuid::new_v4();

        let client_id = 42;
        state.insert_user_client_mapping(user_id, client_id);
        assert_eq!(state.user_to_client.get(&user_id), Some(&client_id));
        let retrieved_client_id = state.get_client_id_by_user(&user_id);
        assert_eq!(retrieved_client_id, Some(client_id));
    }
}
