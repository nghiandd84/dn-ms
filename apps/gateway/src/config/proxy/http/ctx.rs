// use std::error::Error;

use pingora::Error;

use crate::{
    config::{
        proxy::http::HeaderBuffer,
        source_config::{Filter, RouterConfig},
    },
    gateway::interceptor::Phase,
};

#[derive(Debug)]
pub struct HttpGatewayCtx {
    pub request_id: Option<String>,
    pub filter: Option<Filter>,
    pub ds_res_header_buffer: HeaderBuffer,
    pub us_req_header_buffer: HeaderBuffer,
}

impl HttpGatewayCtx {
    pub fn new() -> Self {
        Self {
            request_id: Some(uuid::Uuid::new_v4().to_string()),
            filter: None,
            ds_res_header_buffer: HeaderBuffer::new(),
            us_req_header_buffer: HeaderBuffer::new(),
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

    pub fn get_filter(&self) -> Option<&Filter> {
        self.filter.as_ref()
    }
}


// Header
