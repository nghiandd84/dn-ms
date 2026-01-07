use async_trait::async_trait;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

#[derive(Debug)]
pub struct RequestIdInterceptor {
    filter: Option<String>,
}

impl RequestIdInterceptor {
    pub fn build(filter: Option<String>) -> Self {
        Self { filter }
    }
}

#[async_trait]
impl Interceptor for RequestIdInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::RequestId
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::PostUpstreamResponse.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn post_upstream_response(&self, session: &mut Session) -> PhaseResult {
        let trace_id = session.trace_id();
        debug!(
            "RequestIdInterceptor setting X-Request-Id header with trace_id: {}",
            trace_id
        );
        session.set_ds_res_header(
            "X-Request-Id".to_string(),
            trace_id.clone().into_bytes().to_vec(),
        );
        Ok(true)
    }
}
