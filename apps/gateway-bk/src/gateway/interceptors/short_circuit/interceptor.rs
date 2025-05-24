use async_trait::async_trait;

use crate::{
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

use super::response_parts::ResponseParts;

pub struct ShortCircuitInterceptor {
    filter: Option<String>,
    response_parts: ResponseParts,
}

impl ShortCircuitInterceptor {
    pub fn build(filter: Option<String>, response_parts: ResponseParts) -> Self {
        Self {
            filter,
            response_parts,
        }
    }
}

#[async_trait]
impl Interceptor for ShortCircuitInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::ShortCircuit
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::UpstreamProxyFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        for (header_name, header_value) in &self.response_parts.header_buffer {
            _session.set_ds_res_header(header_name.clone(), header_value.clone());
        }

        if let Some(status_code) = self.response_parts.status_code {
            _session.set_res_status(status_code);
        }

        Ok(true)
    }
}
