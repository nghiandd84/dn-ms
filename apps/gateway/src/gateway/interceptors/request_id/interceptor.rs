use async_trait::async_trait;

use crate::{
    config::proxy::http::Session,
    error::GatewayResult,
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask},
};

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
    fn name(&self) -> InterceptorName {
        InterceptorName::RequestId
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::Init.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn init(&self, _session: &mut Session) -> GatewayResult<()> {
        let req_id = uuid::Uuid::new_v4();
        let req_id = req_id.to_string().as_bytes().to_vec();
        // TODO send req_id to upstream and downstream
        Ok(())
    }
}
