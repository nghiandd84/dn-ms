use async_trait::async_trait;

use crate::{
    error::DakiaResult,
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask},
    proxy::http::Session,
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

    async fn init(&self, _session: &mut Session) -> DakiaResult<()> {
        let req_id = uuid::Uuid::new_v4();
        let req_id = req_id.to_string().as_bytes().to_vec();
        _session.set_ds_res_header("X-Request-Id".to_string(), req_id.clone());
        _session.set_us_req_header("X-Request-Id".to_string(), req_id);
        Ok(())
    }
}
