use async_trait::async_trait;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

#[derive(Debug)]
pub struct TokenAuthInterceptor {
    filter: Option<String>,
}

impl TokenAuthInterceptor {
    pub fn build(filter: Option<String>) -> Self {
        Self { filter }
    }
}

#[async_trait]
impl Interceptor for TokenAuthInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::TokenAuth
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        // TODO extract token from request headers and validate
        let token = session.ds_req_header("Authorization");
        if token.is_none() {
            debug!("No Authorization header found");
            // Here you might want to set an error response in the session
            return Ok(false);
        }
        let token = token.unwrap();
        debug!("Extracted token: {:?}", token);
        Ok(true)
    }
}
