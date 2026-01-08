use async_trait::async_trait;
use tracing::debug;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

use features_auth_remote::TokenService;

#[derive(Debug)]
pub struct TokenAuthConfig {
    pub verify_endpoint: Option<String>,
    pub use_auth_service: bool,
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

    #[tracing::instrument(name = "TokenAuthInterceptor::request_filter", skip(self, session))]
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
        let is_use_auth_service = self.token_auth_config.use_auth_service;
        if is_use_auth_service {
            debug!("Using auth service to verify token");
            let verify_data = TokenService::validate_token(token).await;
            match verify_data {
                Ok(valid) => {
                    debug!("Token is valid: {:?}", valid);
                    // You can set user info in session here if needed
                    return Ok(false);
                }
                Err(e) => {
                    debug!("Token validation failed: {:?}", e);
                    return Ok(true);
                }
            }
        }

        // TODO Call verify_endpoint to validate the token
        Ok(false)
    }
}
