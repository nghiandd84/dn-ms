use async_trait::async_trait;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

#[derive(Debug)]
pub struct TokenAuthConfig {
    pub verify_endpoint: String,
}

#[derive(Debug)]
pub struct TokenAuthInterceptor {
    filter: Option<String>,
    token_auth_config: TokenAuthConfig,
}

impl TokenAuthInterceptor {
    pub fn build(token_auth_config: TokenAuthConfig, filter: Option<String>) -> Self {
        Self {
            filter,
            token_auth_config,
        }
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
            return Ok(true);
        }
        let token = token.unwrap();
        debug!("Extracted token: {:?}", token);
        debug!(
            "TokenAuthInterceptor with filter {:?} and config {:?}",
            self.filter, self.token_auth_config
        );
        // TODO Call verify_endpoint to validate the token
        Ok(false)
    }

}
