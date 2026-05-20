use std::sync::Arc;
use std::time::Duration;

use shared_shared_data_cache::cache::Cache;

use crate::{
    config::source_config::InterceptorConfig,
    error::GatewayResult,
    gateway::{
        interceptor::Interceptor,
        interceptor_builder::InterceptorBuilder,
    },
};

use super::{
    interceptor::{AnomalyDetectorConfig, AnomalyDetectorInterceptor},
    rules::{
        auth_brute_force::AuthBruteForceRule,
        duplicate_payload::DuplicatePayloadRule,
        endpoint_scan::EndpointScanRule,
        payload_size::PayloadSizeRule,
        rapid_path_switch::RapidPathSwitchRule,
        DetectionRule,
    },
};

#[derive(Default)]
pub struct AnomalyDetectorInterceptorBuilder;

impl InterceptorBuilder for AnomalyDetectorInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        let config_map = interceptor_config.config.unwrap_or_default();

        let redis_url = config_map
            .get("redis_url")
            .cloned()
            .unwrap_or_else(|| "redis://127.0.0.1/".to_string());

        let cfg = AnomalyDetectorConfig {
            redis_url: redis_url.clone(),
            max_payload_size: parse_or(&config_map, "max_payload_size", 1_048_576),
            duplicate_threshold: parse_or(&config_map, "duplicate_threshold", 5),
            duplicate_multi_ip_threshold: parse_or(&config_map, "duplicate_multi_ip_threshold", 10),
            duplicate_window: parse_or(&config_map, "duplicate_window", 60),
            max_404_count: parse_or(&config_map, "max_404_count", 10),
            not_found_window: parse_or(&config_map, "not_found_window", 60),
            max_auth_failures: parse_or(&config_map, "max_auth_failures", 5),
            auth_window: parse_or(&config_map, "auth_window", 300),
            block_duration: parse_or(&config_map, "block_duration", 600),
            max_distinct_paths: parse_or(&config_map, "max_distinct_paths", 50),
            path_window: parse_or(&config_map, "path_window", 60),
        };

        let cache = Cache::<String, String>::new(&redis_url, "anomaly")
            .map_err(|_| crate::error::Error::from_str("Failed to connect to Redis"))?;

        let rules: Vec<Arc<dyn DetectionRule>> = vec![
            Arc::new(PayloadSizeRule { max_size: cfg.max_payload_size }),
            Arc::new(DuplicatePayloadRule {
                threshold: cfg.duplicate_threshold,
                multi_ip_threshold: cfg.duplicate_multi_ip_threshold,
                window: Duration::from_secs(cfg.duplicate_window),
            }),
            Arc::new(EndpointScanRule {
                max_404_count: cfg.max_404_count,
                window: Duration::from_secs(cfg.not_found_window),
                block_duration: Duration::from_secs(cfg.block_duration),
            }),
            Arc::new(AuthBruteForceRule {
                max_failures: cfg.max_auth_failures,
                window: Duration::from_secs(cfg.auth_window),
                block_duration: Duration::from_secs(cfg.block_duration),
            }),
            Arc::new(RapidPathSwitchRule {
                max_distinct_paths: cfg.max_distinct_paths,
                window: Duration::from_secs(cfg.path_window),
                block_duration: Duration::from_secs(cfg.block_duration),
            }),
        ];

        let filter = interceptor_config.filter;
        let interceptor = AnomalyDetectorInterceptor::build(filter, cfg, cache, rules);

        Ok(Arc::new(interceptor))
    }
}

fn parse_or(map: &std::collections::HashMap<String, String>, key: &str, default: u64) -> u64 {
    map.get(key).and_then(|v| v.parse().ok()).unwrap_or(default)
}
