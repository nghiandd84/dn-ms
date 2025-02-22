use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DownstreamConfig {
    pub host: String,
    pub port: Option<u16>,
}

impl DownstreamConfig {
    pub fn get_formatted_address(&self) -> String {
        match self.port {
            Some(port) => format!("{}:{}", self.host, port),
            None => self.host.clone(),
        }
    }
}
