use async_trait::async_trait;
use bytes::Bytes;
use pingora_http::ResponseHeader;
use sha2::{Digest, Sha256};
use shared_shared_data_cache::cache::Cache;
use std::time::Duration;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

use super::{DetectionRule, RequestContext, Violation};

pub struct DuplicatePayloadRule {
    pub threshold: u64,
    pub multi_ip_threshold: u64,
    pub window: Duration,
}

#[async_trait]
impl DetectionRule for DuplicatePayloadRule {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation> {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}:{}", ctx.method, ctx.path, ctx.content_length));
        let hash = format!("{:x}", hasher.finalize());

        // Check same-client duplicate
        let client_key = format!("{}:payload:{}", ctx.client_ip, hash);
        let count = match cache.get(&client_key) {
            Ok(Some(v)) => v.parse::<u64>().unwrap_or(0),
            _ => 0,
        };
        let new_count = count + 1;
        let _ = cache.insert(client_key, new_count.to_string(), Some(self.window));

        if new_count > self.threshold {
            return Some(Violation {
                rule: "duplicate_payload",
                reason: format!("Duplicate payload detected {} times from same client", new_count),
                status_code: 429,
            });
        }

        // Check multi-IP duplicate
        let ip_set_key = format!("payload:{}:ips", hash);
        let ips = match cache.get(&ip_set_key) {
            Ok(Some(v)) => v,
            _ => String::new(),
        };
        let mut ip_list: Vec<String> = if ips.is_empty() {
            vec![]
        } else {
            ips.split(',').map(|s| s.to_string()).collect()
        };
        if !ip_list.contains(&ctx.client_ip) {
            ip_list.push(ctx.client_ip.clone());
        }
        let _ = cache.insert(ip_set_key, ip_list.join(","), Some(self.window));

        if ip_list.len() as u64 > self.multi_ip_threshold {
            return Some(Violation {
                rule: "duplicate_payload_multi_ip",
                reason: format!("Same payload from {} distinct IPs", ip_list.len()),
                status_code: 429,
            });
        }

        None
    }

    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult {
        let psession = session.get_psession();
        let mut resp = ResponseHeader::build(http::StatusCode::TOO_MANY_REQUESTS, None).unwrap();
        let _ = resp.insert_header("Content-Type", "text/plain");
        psession.set_keepalive(None);
        let _ = psession.write_response_header(Box::new(resp), false).await;
        let _ = psession.write_response_body(Some(Bytes::from(violation.reason.clone())), true).await;
        Ok(true)
    }
}
