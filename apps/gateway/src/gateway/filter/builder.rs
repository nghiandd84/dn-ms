use crate::{
    config::source_config::GatewayConfig, error::DakiaResult, gateway::filter::query2filter,
    qe::query::extract_key_str_or_err, shared::mutable_registry::Registry,
};

use super::Filter;

pub fn build_filter_registry(gateway_config: &mut GatewayConfig) -> DakiaResult<Registry<Filter>> {
    let mut registry: Registry<Filter> = Registry::build();

    for filter_config in &mut gateway_config.filters {
        let filter_name = extract_key_str_or_err(&filter_config, "name")?.to_string();
        filter_config.remove("name");

        let filter = query2filter(filter_config)?;
        registry.add(filter_name, filter);
    }

    Ok(registry)
}
