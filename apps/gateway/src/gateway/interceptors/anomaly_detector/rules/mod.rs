use async_trait::async_trait;
use shared_shared_data_cache::cache::Cache;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

pub mod payload_size;
pub mod duplicate_payload;
pub mod endpoint_scan;
pub mod auth_brute_force;
pub mod rapid_path_switch;

pub struct RequestContext {
    pub client_ip: String,
    pub path: String,
    pub method: String,
    pub content_length: u64,
}

pub struct Violation {
    pub rule: &'static str,
    pub reason: String,
    pub status_code: u16,
}

#[async_trait]
pub trait DetectionRule: Send + Sync {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation>;

    async fn post_response(
        &self,
        _ctx: &RequestContext,
        _session: &mut Session,
        _cache: &Cache<String, String>,
    ) {
    }

    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult;
}
