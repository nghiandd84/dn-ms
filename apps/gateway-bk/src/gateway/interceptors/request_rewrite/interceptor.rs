use async_trait::async_trait;
use http::Uri;

use crate::{
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

use super::rewrite_parts::RewriteParts;

pub struct RequestRewriteInterceptor {
    filter: Option<String>,
    rewrite_parts: RewriteParts,
}

impl RequestRewriteInterceptor {
    pub fn build(filter: Option<String>, rewrite_parts: RewriteParts) -> Self {
        Self {
            filter,
            rewrite_parts,
        }
    }
}

#[async_trait]
impl Interceptor for RequestRewriteInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::RequestRewrite
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::PreUpstreamRequest.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn pre_upstream_request(&self, _session: &mut Session) -> PhaseResult {
        for (header_name, header_value) in &self.rewrite_parts.header_buffer {
            _session.set_us_req_header(header_name.clone(), header_value.clone());
        }

        if let Some(path) = &self.rewrite_parts.path {
            let builder = Uri::builder().path_and_query(path.as_slice());

            _session.set_us_req_uri(builder.build()?)?;
        }

        Ok(false)
    }
}
