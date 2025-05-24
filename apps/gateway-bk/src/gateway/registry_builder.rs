use std::sync::Arc;

use crate::{
    config::source_config::GatewayConfig,
    error::DakiaResult,
    shared::{
        pattern_matcher::Pcre2PatternMatcher,
        pattern_registry::{PatternRegistry, PatternRegistryType},
        registry::Registry,
    },
};

use super::lb::{build_lb, LbRegistryType, LoadBalancerRegistry};

pub async fn build_ds_host_pattern_registry(
    gateway_config: &GatewayConfig,
) -> DakiaResult<PatternRegistryType> {
    let pattern_registry = PatternRegistry::build();
    for ds in &gateway_config.downstreams {
        let ds_addr = ds.get_formatted_address();
        let pcre2pattern_matcher = Pcre2PatternMatcher::build(&ds_addr)?;
        let _ = pattern_registry
            .register(ds_addr, Arc::new(pcre2pattern_matcher))
            .await;
    }

    Ok(Arc::new(pattern_registry))
}

pub async fn build_lb_registry(gateway_config: &GatewayConfig) -> DakiaResult<LbRegistryType> {
    let lb_registry = LoadBalancerRegistry::build();
    for upstream_config in &gateway_config.upstreams {
        let lb = build_lb(upstream_config)?;
        let arc_lb = Arc::new(lb);

        let _ = lb_registry
            .register(upstream_config.name.clone(), arc_lb.clone())
            .await;

        if upstream_config.default {
            let _ = lb_registry.register("default".to_string(), arc_lb).await;
        }
    }

    Ok(Arc::new(lb_registry))
}
