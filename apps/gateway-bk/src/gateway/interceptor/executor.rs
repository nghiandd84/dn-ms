use std::sync::Arc;

use tracing::trace;

use crate::{error::DakiaResult, gateway::filter::exec_filter, proxy::http::Session};

use super::{is_hook_enabled, is_phase_enabled, Hook, Interceptor, Phase, PhaseResult};

fn match_filter<'a>(filter_name: &Option<String>, session: &Session<'a>) -> DakiaResult<bool> {
    match filter_name {
        Some(filter_name) => {
            let filter =
                session.ctx().gateway_state.filter(filter_name).expect(
                    format!("Unexpected error, {filter_name} filter not found...").as_str(),
                );

            Ok(exec_filter(filter, session)?)
        }
        None => {
            trace!("No filter specified, defaulting to match as true.");
            Ok(true)
        }
    }
}

pub async fn exec_hook<'a>(cur_hook: Hook, session: &mut Session<'a>) -> PhaseResult {
    let gateway_state = session.ctx().gateway_state.clone();
    let interceptors = gateway_state.interceptors();

    for interceptor in interceptors {
        let is_hook_enabled = is_hook_enabled(interceptor.hook_mask(), &cur_hook);
        // TODO: store filter matching status inside RwLock<HashMap<&str,bool>> inside session.ctx() to avoid doing heavy computation while filtering the same logic
        let is_filter_matched = match_filter(interceptor.filter(), session)?;
        if !is_hook_enabled || !is_filter_matched {
            continue;
        }

        match cur_hook {
            Hook::PreDownstreamResponseHeaderFlush => {
                interceptor.pre_downstream_response_hook(session).await
            }
        }?;
    }

    Ok(false)
}

async fn execute_interceptor_phase<'a>(
    interceptor: &Arc<dyn Interceptor>,
    session: &mut Session<'a>,
) -> PhaseResult {
    let phase = session.phase();
    let is_phase_enabled = is_phase_enabled(interceptor.phase_mask(), phase);

    trace!(
        "Executing interceptor {:?} phase: {:?}, enabled: {}",
        interceptor.name(),
        phase,
        is_phase_enabled,
    );

    // TODO: store filter matching status inside RwLock<HashMap<&str,bool>> inside session.ctx() to avoid doing heavy computation while filtering the same logic
    let is_filter_matched = match_filter(interceptor.filter(), session)?;

    trace!(
        "Filter result for interceptor {:?} filter matched: {}",
        interceptor.name(),
        is_filter_matched
    );

    if !is_phase_enabled || !is_filter_matched {
        return Ok(false); // false - continue to other phase or interceptor
    }

    match phase {
        Phase::Init => {
            interceptor.init(session).await?;
            Ok(false)
        }
        Phase::RequestFilter => interceptor.request_filter(session).await,
        Phase::UpstreamProxyFilter => interceptor.upstream_proxy_filter(session).await,
        Phase::UpstreamPeerSelection => todo!(), // no such requirement as of now
        Phase::PreUpstreamRequest => interceptor.pre_upstream_request(session).await,
        Phase::PostUpstreamResponse => interceptor.post_upstream_response(session).await,
        Phase::PreDownstreamResponse => interceptor.pre_downstream_response(session).await,
    }
}

pub async fn exec_phase<'a>(session: &mut Session<'a>) -> PhaseResult {
    let gateway_state = session.ctx().gateway_state.clone();
    let interceptors = gateway_state.interceptors();

    for interceptor in interceptors {
        let phase_result = execute_interceptor_phase(interceptor, session).await?;
        if phase_result {
            return Ok(true);
        }
    }

    Ok(false)
}
