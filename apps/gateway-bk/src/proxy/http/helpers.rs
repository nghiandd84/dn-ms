use pingora::lb::Backend;

use crate::{
    config::{source_config::GatewayConfig, InetAddress},
    error::{DakiaError, DakiaResult},
    shared::pattern_registry::PatternRegistryType,
};

fn get_ds_addrs(gateway_config: &GatewayConfig) -> Vec<String> {
    // safe to unwrap
    gateway_config
        .downstreams
        .iter()
        .map(|d| d.get_formatted_address())
        .collect()
}

pub async fn is_valid_ds_host(
    dakia_config: &GatewayConfig,
    ds_host_pattern_registry: &PatternRegistryType,
    ds_host: &[u8],
) -> DakiaResult<bool> {
    let ds_addrs = get_ds_addrs(dakia_config);

    for ds_addr in ds_addrs {
        let pattern = ds_host_pattern_registry
            .get(&ds_addr)
            .await?
            .ok_or(DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(crate::error::ImmutStr::Owned(
                    "compiled pattern for downstream not found"
                        .to_string()
                        .into_boxed_str(),
                )),
                None,
            ))?;

        let is_matched: bool = pattern.is_match(ds_host).map_err(|e| {
            println!("{}", e);
            DakiaError::create_internal()
        })?;

        if is_matched {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_inet_addr_from_backend(backend: &Backend) -> InetAddress {
    let addr = backend.addr.clone().to_string();
    let parts: Vec<&str> = addr.split(":").collect();

    InetAddress {
        host: parts[0].to_owned(),
        // TODO: handle unwrap
        port: parts[1].parse().unwrap(),
    }
}
