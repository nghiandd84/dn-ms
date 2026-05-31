use std::sync::Arc;

use async_trait::async_trait;
use tracing::{debug, error};

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
        Phase::Init.mask()
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
    phase: &Phase,
) -> PhaseResult {
    for interceptor in interceptors.iter() {
        let execute = execute_interceptor(interceptor, session, phase).await;
        match execute {
            Ok(exec) => {
                if exec {
                    error!(
                        "Fail to execute Interceptor {:?} at filter {:?}.",
                        interceptor.interceptor_type(),
                        session.get_filter()
                    );
                    return Ok(true);
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(false)
}
async fn execute_interceptor<'a>(
    interceptor: &Arc<dyn Interceptor>,
    session: &mut Session<'a>,
    phase: &Phase,
) -> PhaseResult {
    match phase {
        Phase::Init => interceptor.init(session).await,
        Phase::RequestFilter => {
            debug!(
                "Executing RequestFilter for interceptor: {:?}",
                interceptor.interceptor_type()
            );
            interceptor.request_filter(session).await
        }
        Phase::PostUpstreamResponse => {
            debug!(
                "Executing PostUpstreamResponse for interceptor: {:?}",
                interceptor.interceptor_type()
            );
            interceptor.post_upstream_response(session).await
        }
        _ => Ok(false),
    }
}
