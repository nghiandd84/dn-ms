use async_trait::async_trait;
use bytes::Bytes;
use pingora_http::ResponseHeader;
use shared_shared_data_cache::cache::Cache;
use std::time::Duration;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

use super::{DetectionRule, RequestContext, Violation};

pub struct EndpointScanRule {
    pub max_404_count: u64,
    pub window: Duration,
    pub block_duration: Duration,
}

#[async_trait]
impl DetectionRule for EndpointScanRule {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation> {
        // Check if IP is already blocked
        let block_key = format!("{}:blocked:scan", ctx.client_ip);
        if let Ok(Some(_)) = cache.get(&block_key) {
            return Some(Violation {
                rule: "endpoint_scan",
                reason: "IP blocked due to endpoint scanning".to_string(),
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
        // Check if upstream returned 404
        let status = session
            .get_psession()
            .response_written()
            .map(|r| r.status.as_u16())
            .unwrap_or(0);

        if status == 404 {
            let key = format!("{}:not_found", ctx.client_ip);
            let count = match cache.get(&key) {
                Ok(Some(v)) => v.parse::<u64>().unwrap_or(0),
                _ => 0,
            };
            let new_count = count + 1;
            let _ = cache.insert(key, new_count.to_string(), Some(self.window));

            if new_count >= self.max_404_count {
                let block_key = format!("{}:blocked:scan", ctx.client_ip);
                let _ = cache.insert(block_key, "1".to_string(), Some(self.block_duration));
            }
        }
    }

    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult {
        let psession = session.get_psession();
        let mut resp = ResponseHeader::build(http::StatusCode::FORBIDDEN, None).unwrap();
        let _ = resp.insert_header("Content-Type", "text/plain");
        psession.set_keepalive(None);
        let _ = psession.write_response_header(Box::new(resp), false).await;
        let _ = psession.write_response_body(Some(Bytes::from(violation.reason.clone())), true).await;
        Ok(true)
    }
}
