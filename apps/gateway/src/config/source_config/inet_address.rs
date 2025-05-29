use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InetAddress {
    pub host: String,
    pub port: u16,
}

impl InetAddress {
    
    pub fn get_formatted_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
