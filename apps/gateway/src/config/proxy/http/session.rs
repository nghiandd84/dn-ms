use http::Uri;
use opentelemetry::trace::TraceContextExt;
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;
use std::mem::take;
use tracing::debug;

use crate::{
    config::source_config::{Filter, PathFilter},
    error::Error,
    gateway::interceptor::{Phase, PhaseResult},
};

use super::ctx::HttpGatewayCtx;

pub struct Session<'a> {
    ctx: &'a mut HttpGatewayCtx,
    phase: Phase,
    psession: &'a mut PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    upstream_response: Option<&'a mut PResponseHeader>,
    ds_header_flushed: bool,
}

impl<'a> Session<'a> {
    pub fn build(phase: Phase, psession: &'a mut PSession, ctx: &'a mut HttpGatewayCtx) -> Self {
        Session {
            ctx,
            phase,
            psession,
            upstream_request: None,
            upstream_response: None,
            ds_header_flushed: false,
        }
    }

    pub fn upstream_request(&mut self, upstream_request: &'a mut PRequestHeader) {
        self.upstream_request = Some(upstream_request);
    }

    pub fn upstream_response(&mut self, upstream_response: &'a mut PResponseHeader) {
        self.upstream_response = Some(upstream_response);
    }
}

impl<'a> Session<'a> {
    pub fn set_us_req_header(&mut self, header_name: String, header_value: Vec<u8>) {
        self.ctx
            .us_req_header_buffer
            .insert(header_name, header_value);
    }

    pub fn set_ds_res_header(&mut self, header_name: String, header_value: Vec<u8>) {
        self.ctx
            .ds_res_header_buffer
            .insert(header_name, header_value);
    }

    pub fn ds_req_header(&self, header_name: &str) -> Option<String> {
        let header_value = self
            .psession
            .as_downstream()
            .req_header()
            .headers
            .get(header_name);

        match header_value {
            Some(value) => Some(String::from_utf8(value.as_bytes().to_vec()).unwrap()),
            None => None,
        }
    }

    pub fn trace_id(&self) -> String {
        let context = self.ctx.span_context.as_ref();
        if context.is_none() {
            return "".to_string();
        }
        let context = context.unwrap().clone();
        let span = context.span();
        let span_context = span.span_context();
        let trace_id = span_context.trace_id().to_string();
        trace_id
    }

    async fn flush_header_to_ds(&mut self) -> PhaseResult {
        debug!("flush_header_to_ds");
        // let mut header = PResponseHeader::build(self.ds_status_code, None).unwrap();
        match self.upstream_response.as_mut() {
            Some(upstream_response) => {
                debug!("Insert header to downstream");
                let headers = take(&mut self.ctx.ds_res_header_buffer);
                for (header_name, header_value) in headers.into_iter() {
                    debug!("Insert header to downstream {} ", header_name);
                    let _insert = upstream_response.insert_header(header_name, header_value);
                }
                Ok(true)
            }
            None => Err(Box::new(Error::from_str(
                "Something went wrong! Upstream headers are not present",
            ))),
        }

        // let headers = take(&mut self.ctx.ds_res_header_buffer);
        // for (header_name, header_value) in headers.into_iter() {
        //     header.insert_header(header_name, header_value)?;
        // }

        // self.psession
        //     .write_response_header(Box::new(header), false)
        //     .await?;

        // Ok(true)
    }

    pub fn flush_us_req_header(&mut self) -> PhaseResult {
        match self.upstream_request.as_mut() {
            Some(upstream_request) => {
                let headers = take(&mut self.ctx.us_req_header_buffer);
                for (header_name, header_value) in headers.into_iter() {
                    debug!("Insert header to upstream {} ", header_name);
                    let _insert = upstream_request.insert_header(header_name, header_value);
                }
                Ok(true)
            }
            None => Err(Box::new(Error::from_str(
                "Something went wrong! Upstream headers are not present",
            ))),
        }
    }

    pub async fn flush_ds_res_header(&mut self) -> PhaseResult {
        if self.ds_header_flushed {
            return Ok(true);
        }

        self.ds_header_flushed = true;

        match self.phase {
            Phase::Init
            | Phase::RequestFilter
            | Phase::UpstreamProxyFilter
            | Phase::PreDownstreamResponse
            | Phase::UpstreamPeerSelection => {
                // self.flush_header_to_ds().await
                Ok(true)
            }
            Phase::PostUpstreamResponse => {
                let _flush = self.flush_header_to_ds().await;
                Ok(true)
            }
            _ => Ok(false),
            // Phase::PreUpstreamRequest => Err(DakiaError::i_explain(format!(
            //     "can not write downstream headers in {} phase",
            //     Phase::PreUpstreamRequest
            // ))),
        }
    }
}

/// Override request path
impl<'a> Session<'a> {
    pub fn ds_req_path(&self) -> &str {
        self.psession.as_downstream().req_header().uri.path()
    }

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

    // pub fn flush_timeout(&mut self, filter: &Filter) -> () {
    //     if let Some(timeout) = filter.timeout {
    //         // self.upstream_request.
    //         // self.psession.set_pro(Duration::from_secs(timeout));

    //         // self.psession.set_timeout(timeout);
    //     }
    // }

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
