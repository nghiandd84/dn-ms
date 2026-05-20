use async_trait::async_trait;
use bytes::Bytes;
use pingora_http::ResponseHeader;
use shared_shared_data_cache::cache::Cache;
use std::time::Duration;

use crate::{config::proxy::http::Session, gateway::interceptor::PhaseResult};

use super::{DetectionRule, RequestContext, Violation};

pub struct RapidPathSwitchRule {
    pub max_distinct_paths: u64,
    pub window: Duration,
    pub block_duration: Duration,
}

#[async_trait]
impl DetectionRule for RapidPathSwitchRule {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation> {
        // Check if already blocked
        let block_key = format!("{}:blocked:paths", ctx.client_ip);
        if let Ok(Some(_)) = cache.get(&block_key) {
            return Some(Violation {
                rule: "rapid_path_switch",
                reason: "IP blocked due to rapid path switching".to_string(),
                status_code: 403,
            });
        }

        // Track distinct paths
        let paths_key = format!("{}:paths", ctx.client_ip);
        let paths = match cache.get(&paths_key) {
            Ok(Some(v)) => v,
            _ => String::new(),
        };
        let mut path_list: Vec<String> = if paths.is_empty() {
            vec![]
        } else {
            paths.split('\n').map(|s| s.to_string()).collect()
        };
        if !path_list.contains(&ctx.path) {
            path_list.push(ctx.path.clone());
        }
        let _ = cache.insert(paths_key, path_list.join("\n"), Some(self.window));

        if path_list.len() as u64 > self.max_distinct_paths {
            let _ = cache.insert(block_key, "1".to_string(), Some(self.block_duration));
            return Some(Violation {
                rule: "rapid_path_switch",
                reason: format!("Accessed {} distinct paths in window", path_list.len()),
                status_code: 403,
            });
        }

        None
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
