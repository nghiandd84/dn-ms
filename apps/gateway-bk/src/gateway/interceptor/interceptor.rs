use async_trait::async_trait;

use crate::{config::source_config::InterceptorConfig, error::DakiaResult, proxy::http::Session};

use super::{HookMask, InterceptorName, PhaseMask};

pub type PhaseResult = DakiaResult<bool>;

#[async_trait]
pub trait Interceptor: Send + Sync {
    fn name(&self) -> InterceptorName;

    fn phase_mask(&self) -> PhaseMask {
        0 // no phase will be executed
    }

    fn hook_mask(&self) -> HookMask {
        0 // no hook will be executed
    }

    fn _init(&mut self, _interceptor_config: &InterceptorConfig) -> DakiaResult<()> {
        Ok(())
    }

    // if there is no filter, it'll be considered as match
    fn filter(&self) -> &Option<String> {
        &None
    }

    async fn init(&self, _session: &mut Session) -> DakiaResult<()> {
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
