use async_trait::async_trait;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    error::GatewayResult,
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
        Phase::Init.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn init(&self, _session: &mut Session) -> PhaseResult {
        let req_id = uuid::Uuid::new_v4().to_string().as_bytes().to_vec();
        // TODO send req_id to upstream and downstream
        debug!(
            "Init RequestIdInterceptor with filter {}",
            self.filter.as_ref().unwrap()
        );
        Ok(true)
    }
}
