use async_trait::async_trait;
use opentelemetry::baggage::{Baggage, BaggageExt};
use opentelemetry::Context;
use tracing::{debug, span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

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

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        let request_path = session.ds_req_path();
        debug!("Request path: {}", request_path);
        if request_path.starts_with("/public/") {
            debug!("Public path, skipping token auth");
            return Ok(false);
        }
        let token = session.ds_req_header("Authorization");
        if token.is_none() {
            debug!("No Authorization header found");
            return Ok(true);
        }
        let token = token.unwrap();
        // Remove Bearer prefix if present
        let token = token.trim_start_matches("Bearer ").to_string();

        let is_use_auth_service = self.token_auth_config.use_auth_service;
        if is_use_auth_service {
            debug!("Using auth service to verify token");
            let span_context = session.get_span_context().as_ref().unwrap().clone();
            debug!("Span context in session: {:?}", span_context);
            let verify_token_span = tracing::info_span!("verify_token");
            let _ = verify_token_span.set_parent(span_context.clone());
            let _entered_span = verify_token_span.enter();
            let verify_data = TokenService::validate_token(token).await;
            let access_token = match verify_data {
                Ok(claim_subject) => claim_subject,
                Err(e) => {
                    debug!("Token validation failed: {:?}", e);
                    return Ok(true);
                }
            };
            debug!("Access token result: {:?}", access_token);
            // Update baggae context of span_context
            let mut baggage = Baggage::new();
            let _ = baggage.insert("user_id", access_token.user_id.to_string());
            let _ = baggage.insert("client_id", access_token.client_id.to_string());
            let _ = baggage.insert("accesses", access_token.access_to_string());
            let updated_span_context = span_context.with_baggage(baggage);
            session.set_us_req_header("Authorization".to_string(), vec![]);
            session.set_span_context(updated_span_context.clone());
            debug!(
                "Updated span context baggage: {:?}",
                updated_span_context.baggage()
            );
        }

        Ok(false)
    }
}
