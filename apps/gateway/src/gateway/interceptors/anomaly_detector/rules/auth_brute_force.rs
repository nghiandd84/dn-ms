use async_trait::async_trait;
use bytes::Bytes;
use pingora_http::ResponseHeader;
use shared_shared_data_cache::cache::Cache;
use std::time::Duration;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

use super::{DetectionRule, RequestContext, Violation};

pub struct AuthBruteForceRule {
    pub max_failures: u64,
    pub window: Duration,
    pub block_duration: Duration,
}

#[async_trait]
impl DetectionRule for AuthBruteForceRule {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation> {
        let block_key = format!("{}:blocked:auth", ctx.client_ip);
        if let Ok(Some(_)) = cache.get(&block_key) {
            return Some(Violation {
                rule: "auth_brute_force",
                reason: "IP blocked due to repeated auth failures".to_string(),
                status_code: 403,
            });
        }
        None
    }

    async fn post_response(
        &self,
        ctx: &RequestContext,
        session: &mut Session,
        cache: &Cache<String, String>,
    ) {
        // Only track auth-related paths
        if !ctx.path.contains("/auth") {
            return;
        }

        let status = session
            .get_psession()
            .response_written()
            .map(|r| r.status.as_u16())
            .unwrap_or(0);

        if status == 401 || status == 403 {
            let key = format!("{}:auth_failures", ctx.client_ip);
            let count = match cache.get(&key) {
                Ok(Some(v)) => v.parse::<u64>().unwrap_or(0),
                _ => 0,
            };
            let new_count = count + 1;
            let _ = cache.insert(key, new_count.to_string(), Some(self.window));

            if new_count >= self.max_failures {
                let block_key = format!("{}:blocked:auth", ctx.client_ip);
                let _ = cache.insert(block_key, "1".to_string(), Some(self.block_duration));
            }
        }
    }

    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult {
        let psession = session.get_psession();
        let mut resp = ResponseHeader::build(http::StatusCode::FORBIDDEN, None).unwrap();
        let _ = resp.insert_header("Content-Type", "text/plain");
        let _ = resp.insert_header("Retry-After", self.block_duration.as_secs().to_string());
        psession.set_keepalive(None);
        let _ = psession.write_response_header(Box::new(resp), false).await;
        let _ = psession.write_response_body(Some(Bytes::from(violation.reason.clone())), true).await;
        Ok(true)
    }
}
