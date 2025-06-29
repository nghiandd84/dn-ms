use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    config::{proxy::http::Session, source_config::InterceptorConfig},
    error::GatewayResult,
    gateway::interceptor::Phase,
};

use super::{InterceptorType, PhaseMask};

pub type PhaseResult = GatewayResult<bool>;

#[async_trait]

pub trait Interceptor: Send + Sync {
    fn interceptor_type(&self) -> InterceptorType;
    fn phase_mask(&self) -> PhaseMask {
        0
    }

    fn _init(&mut self, _interceptor_config: &InterceptorConfig) -> GatewayResult<()> {
        Ok(())
    }

    fn filter(&self) -> &Option<String> {
        &None
    }

    async fn init(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
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

pub async fn execute_interceptors<'a>(
    interceptors: &Vec<Arc<dyn Interceptor>>,
    session: &mut Session<'a>,
) -> PhaseResult {
    for interceptor in interceptors.iter() {
        let _ = execute_interceptor(interceptor, session).await;
    }
    Ok(true)
}
async fn execute_interceptor<'a>(
    interceptor: &Arc<dyn Interceptor>,
    session: &mut Session<'a>,
) -> PhaseResult {
    let mask_phase = interceptor.phase_mask();

    match mask_phase {
        m if m == Phase::Init.mask() => {
            // handle Init phase
            interceptor.init(session).await
        }
        m if m == Phase::RequestFilter.mask() => {
            // handle RequestFilter phase
            interceptor.request_filter(session).await
        }
        _ => Ok(true),
    }
}
