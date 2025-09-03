use serde::{Deserialize, Serialize};

use crate::types::Clients;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NotificationCacheState {}

#[derive(Clone)]
pub struct NotificationState {
    clients: Clients,
}

impl NotificationState {
    pub fn new(clients: Clients) -> Self {
        Self { clients }
    }

    pub fn get_clients(&self) -> Clients {
        self.clients.clone()
    }
}
