use rs_consul::Consul;
use std::sync::{LazyLock, Mutex};
use tracing::debug;

use shared_shared_data_core::roundrobin::RoundRobin;
use shared_shared_macro::RemoteService;


#[derive(Debug, RemoteService)]
#[remote(name(auth_service))]
pub struct TokenService {}

impl TokenService {
    pub async fn validate_token(&self, token: String) -> Result<bool, String> {
        // Self::update();
        let data: (String, u16) = Self::get_instance().unwrap();
        let (ip, port) = data;
        println!("Connecting to auth service at {}:{}", ip, port);
        // let service_name = TokenService::service_name();
        // self.new_method();
        // Simulate token validation logic
        if token == "valid_token" {
            Ok(true)
        } else {
            Err("Invalid token".into())
        }
    }
}
