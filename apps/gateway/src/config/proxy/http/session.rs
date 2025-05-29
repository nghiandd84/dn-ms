use http::Uri;
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::config::source_config::{Filter, PathFilter};

use super::ctx::HttpGatewayCtx;

pub struct Session<'a> {
    psession: &'a mut PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    upstream_response: Option<&'a mut PResponseHeader>,
    ctx: &'a mut HttpGatewayCtx,
}

impl<'a> Session<'a> {
    pub fn build(psession: &'a mut PSession, ctx: &'a mut HttpGatewayCtx) -> Self {
        Session {
            ctx,
            psession,
            upstream_request: None,
            upstream_response: None,
        }
    }

    pub fn upstream_request(&mut self, upstream_request: &'a mut PRequestHeader) {
        self.upstream_request = Some(upstream_request);
    }

    pub fn upstream_response(&mut self, upstream_response: &'a mut PResponseHeader) {
        self.upstream_response = Some(upstream_response);
    }
}

/// Override request path
impl<'a> Session<'a> {
    pub fn ds_req_path(&self) -> &str {
        self.psession.as_downstream().req_header().uri.path()
    }

    // pub fn set_us_req_uri(&mut self, uri: Uri) -> () {
    //     match self.upstream_request.as_mut() {
    //         Some(upstream_request) => {
    //             upstream_request.set_uri(uri);
    //         }
    //         None => {}
    //     }
    // }

    pub fn flush_path_and_query(&mut self, filter: &Filter) -> () {
        let current_path_and_query = self.path_and_query();
        let mut new_path_str = current_path_and_query.to_string();
        let filter_path = filter.path.clone().unwrap();
        match filter_path {
            PathFilter::StartWith { value } => {
                new_path_str.replace_range(0..value.len(), "");
            }
            PathFilter::EndWith { value } => {}
        }

        let uri = new_path_str.parse::<Uri>().unwrap();
        self.psession.req_header_mut().set_uri(uri);
    }

    fn path_and_query(&self) -> &str {
        let path_and_query = self
            .psession
            .req_header()
            .uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");
        path_and_query
    }
}
