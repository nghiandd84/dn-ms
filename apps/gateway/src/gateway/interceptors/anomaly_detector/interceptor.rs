use std::sync::Arc;

use async_trait::async_trait;
use opentelemetry::{global, KeyValue};
use sha2::{Digest, Sha256};
use shared_shared_data_cache::cache::Cache;
use tracing::warn;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

use super::rules::{DetectionRule, RequestContext};

pub struct AnomalyDetectorConfig {
    pub redis_url: String,
    pub max_payload_size: u64,
    pub duplicate_threshold: u64,
    pub duplicate_multi_ip_threshold: u64,
    pub duplicate_window: u64,
    pub max_404_count: u64,
    pub not_found_window: u64,
    pub max_auth_failures: u64,
    pub auth_window: u64,
    pub block_duration: u64,
    pub max_distinct_paths: u64,
    pub path_window: u64,
}

pub struct AnomalyDetectorInterceptor {
    pub(crate) filter: Option<String>,
    pub(crate) config: AnomalyDetectorConfig,
    pub(crate) cache: Cache<String, String>,
    pub(crate) rules: Vec<Arc<dyn DetectionRule>>,
}

impl AnomalyDetectorInterceptor {
    pub fn build(
        filter: Option<String>,
        config: AnomalyDetectorConfig,
        cache: Cache<String, String>,
        rules: Vec<Arc<dyn DetectionRule>>,
    ) -> Self {
        Self { filter, config, cache, rules }
    }

    /// Extracts a client identity using a priority-based strategy:
    /// 1. Browser fingerprint from X-Client-Fingerprint header (most accurate)
    /// 2. Authenticated user ID from Authorization token
    /// 3. Composite fingerprint: IP + User-Agent + Accept-Language (best effort)
    /// 4. Fallback: IP only
    fn extract_client_identity(&self, session: &mut Session) -> String {
        let ip = session
            .get_psession()
            .client_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Priority 1: Client-side browser fingerprint
        if let Some(fp) = session.get_req_header("x-client-fingerprint") {
            if fp.len() >= 16 && fp.chars().all(|c| c.is_ascii_hexdigit()) {
                return format!("cfp:{}", fp);
            }
        }

        // Priority 2: Use user ID from JWT token if available
        if let Some(auth) = session.get_req_header("authorization") {
            if auth.starts_with("Bearer ") {
                let mut hasher = Sha256::new();
                hasher.update(auth[7..].as_bytes());
                return format!("user:{:x}", hasher.finalize());
            }
        }

        // Priority 3: Composite fingerprint from headers
        let user_agent = session.get_req_header("user-agent").unwrap_or_default();
        let accept_lang = session.get_req_header("accept-language").unwrap_or_default();

        if !user_agent.is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(format!("{}:{}:{}", ip, user_agent, accept_lang));
            return format!("fp:{:x}", hasher.finalize());
        }

        // Priority 4: Fallback to IP
        format!("ip:{}", ip)
    }
}

#[async_trait]
impl Interceptor for AnomalyDetectorInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::AnomalyDetector
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask() | Phase::PostUpstreamResponse.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        let client_id = self.extract_client_identity(session);
        let path = session.ds_req_path().to_string();

        // Check manual block first (set by anomaly_detector API)
        let manual_block_key = format!("{}:blocked:manual", client_id);
        if let Ok(Some(_)) = self.cache.get(&manual_block_key) {
            warn!(
                client_id = %client_id,
                rule_name = "manual_block",
                path = %path,
                action = "block",
                "Manually blocked client"
            );
            let psession = session.get_psession();
            let mut resp = pingora_http::ResponseHeader::build(http::StatusCode::FORBIDDEN, None).unwrap();
            let _ = resp.insert_header("Content-Type", "text/plain");
            psession.set_keepalive(None);
            let _ = psession.write_response_header(Box::new(resp), false).await;
            let _ = psession.write_response_body(Some(bytes::Bytes::from("Forbidden")), true).await;
            return Ok(true);
        }

        let method = session
            .get_req_header(":method")
            .unwrap_or_default();
        let content_length = session
            .get_req_header("content-length")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let ctx = RequestContext {
            client_ip: client_id.clone(),
            path: path.clone(),
            method,
            content_length,
        };

        let meter = global::meter("gateway_anomaly_detector");
        let counter = meter.u64_counter("gateway.anomaly.blocked").build();

        for rule in &self.rules {
            if let Some(violation) = rule.check(&ctx, &self.cache).await {
                warn!(
                    client_id = %client_id,
                    rule_name = violation.rule,
                    violation_type = violation.rule,
                    path = %path,
                    action = "block",
                    reason = %violation.reason,
                    "Anomaly detected"
                );
                counter.add(1, &[KeyValue::new("rule", violation.rule)]);
                return rule.respond(session, &violation).await;
            }
        }

        Ok(false)
    }

    async fn post_upstream_response(&self, session: &mut Session) -> PhaseResult {
        let client_id = self.extract_client_identity(session);
        let path = session.ds_req_path().to_string();

        let ctx = RequestContext {
            client_ip: client_id,
            path,
            method: String::new(),
            content_length: 0,
        };

        for rule in &self.rules {
            rule.post_response(&ctx, session, &self.cache).await;
        }

        Ok(false)
    }
}
