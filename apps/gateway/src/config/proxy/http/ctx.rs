use opentelemetry::context::Context;
use pingora::Error;

use crate::config::{
    proxy::http::HeaderBuffer,
    source_config::{Filter, RouterConfig},
};

#[derive(Debug)]
pub struct HttpGatewayCtx {
    pub span_context: Option<Context>,
    pub filter: Option<Filter>,
    pub ds_res_header_buffer: HeaderBuffer,
    pub us_req_header_buffer: HeaderBuffer,
}

impl HttpGatewayCtx {
    pub fn new() -> Self {
        Self {
            filter: None,
            span_context: None,
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
    pub fn set_span_context(&mut self, span_context: Context) {
        self.span_context = Some(span_context);
    }

    pub fn get_span_context(&self) -> Option<&Context> {
        self.span_context.as_ref()
    }
}

// Header
