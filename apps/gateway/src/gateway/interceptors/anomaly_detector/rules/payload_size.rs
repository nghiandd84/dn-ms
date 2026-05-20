use async_trait::async_trait;
use bytes::Bytes;
use pingora_http::ResponseHeader;
use shared_shared_data_cache::cache::Cache;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

use super::{DetectionRule, RequestContext, Violation};

pub struct PayloadSizeRule {
    pub max_size: u64,
}

#[async_trait]
impl DetectionRule for PayloadSizeRule {
    async fn check(&self, ctx: &RequestContext, _cache: &Cache<String, String>) -> Option<Violation> {
        if ctx.content_length > self.max_size {
            return Some(Violation {
                rule: "payload_size",
                reason: format!("Payload size {} exceeds max {}", ctx.content_length, self.max_size),
                status_code: 413,
            });
        }
        None
    }

    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult {
        let psession = session.get_psession();
        let mut resp = ResponseHeader::build(http::StatusCode::PAYLOAD_TOO_LARGE, None).unwrap();
        let _ = resp.insert_header("Content-Type", "text/plain");
        psession.set_keepalive(None);
        let _ = psession.write_response_header(Box::new(resp), false).await;
        let _ = psession.write_response_body(Some(Bytes::from(violation.reason.clone())), true).await;
        Ok(true)
    }
}
