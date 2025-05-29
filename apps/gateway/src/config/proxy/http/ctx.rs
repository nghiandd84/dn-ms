// use std::error::Error;

use pingora::Error;

use crate::config::source_config::{Filter, RouterConfig};

#[derive(Debug)]
pub struct HttpGatewayCtx {
    pub request_id: Option<String>,
    pub filter: Option<Filter>,
}

impl HttpGatewayCtx {
    pub fn new() -> Self {
        Self {
            request_id: Some(uuid::Uuid::new_v4().to_string()),
            filter: None,
        }
    }
}

impl HttpGatewayCtx {
    pub fn find_router_config<'a>() -> Result<RouterConfig, Box<Error>> {
        todo!();
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = Some(filter);
    }
}
