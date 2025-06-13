use async_trait::async_trait;

use crate::{
    config::{proxy::http::Session, source_config::InterceptorConfig},
    error::GatewayResult,
};

use super::{InterceptorName, PhaseMask};

pub type PhaseResult = GatewayResult<bool>;

#[async_trait]
pub trait Interceptor: Send + Sync {
    fn name(&self) -> InterceptorName;
    fn phase_mask(&self) -> PhaseMask {
        0
    }

    fn _init(&mut self, _interceptor_config: &InterceptorConfig) -> GatewayResult<()> {
        Ok(())
    }

    fn filter(&self) -> &Option<String> {
        &None
    }

    async fn init(&self, _session: &mut Session) -> GatewayResult<()> {
        Ok(())
    }

    async fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    async fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    async fn pre_upstream_request(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    async fn post_upstream_response(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    async fn pre_downstream_response(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    async fn pre_downstream_response_hook(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }
}
