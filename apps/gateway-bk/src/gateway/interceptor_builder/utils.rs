use std::{collections::HashMap, sync::Arc};

use tracing::debug;

use crate::{
    config::source_config::{GatewayConfig, InterceptorConfig},
    error::{DakiaError, DakiaResult},
    gateway::interceptor::Interceptor,
    proxy::http::HeaderBuffer,
    qe::query::{self, Query, Value},
};

use super::InterceptorBuilderRegistry;

fn pull_str_or_err<'a>(qkey: &'a str, qval: &'a Value) -> DakiaResult<&'a str> {
    let dakia_error = DakiaError::i_explain(format!("expected string value for header {}", qkey));
    let err = Err(dakia_error);
    match qval {
        Value::Scaler(scaler) => match scaler {
            query::Scaler::String(hval) => Ok(hval),
            _ => err,
        },
        _ => err,
    }
}

fn pull_header_bytes_or_err<'a>(qkey: &'a str, qval: &'a Value) -> DakiaResult<Vec<u8>> {
    let hval = pull_str_or_err(&qkey, &qval)?;
    let hval_bytes = hval.as_bytes();
    Ok(hval_bytes.to_owned())
}

pub fn extract_headers(intercept_query: &Query) -> DakiaResult<(HeaderBuffer, HeaderBuffer)> {
    let mut ds_res_header_buf: HeaderBuffer = HashMap::new();
    let mut us_req_header_buf: HeaderBuffer = HashMap::new();

    for (qkey, qval) in intercept_query {
        if qkey.starts_with("ds.res.header") {
            let hkey = qkey.replace("ds.res.header.", "");
            let hval_bytes = pull_header_bytes_or_err(qkey, qval)?;
            ds_res_header_buf.insert(hkey, hval_bytes);
        }

        if qkey.starts_with("us.req.header") {
            let hkey = qkey.replace("ds.req.header.", "");
            let hval_bytes = pull_header_bytes_or_err(qkey, qval)?;
            us_req_header_buf.insert(hkey, hval_bytes);
        }
    }

    Ok((ds_res_header_buf, us_req_header_buf))
}

pub fn build_interceptor(
    interceptor_config: &InterceptorConfig,
    interceptor_builder_registry: &InterceptorBuilderRegistry,
) -> DakiaResult<Arc<dyn Interceptor>> {
    let interceptor_name = &interceptor_config.name;
    let builder = interceptor_builder_registry.registry.get(interceptor_name);

    let interceptor = match builder {
        Some(builder) => builder.build(interceptor_config.clone())?,
        None => {
            return Err(DakiaError::i_explain(format!(
                "Invalid interceptor name {:?}. No such interceptor exists",
                interceptor_name.as_str()
            )))
        }
    };
    Ok(interceptor)
}

pub fn build_interceptors(
    gateway_config: &GatewayConfig,
    interceptor_builder_registry: &InterceptorBuilderRegistry,
) -> DakiaResult<Vec<Arc<dyn Interceptor>>> {
    let mut interceptors: Vec<Arc<dyn Interceptor>> = vec![];

    for interceptor_config in &gateway_config.interceptors {
        debug!(
            "Initializing interceptor: {:?} (enabled: {})",
            interceptor_config.name, interceptor_config.enabled
        );

        if !interceptor_config.enabled {
            continue;
        }

        let interceptor = build_interceptor(interceptor_config, interceptor_builder_registry)?;
        interceptors.push(interceptor);
    }

    Ok(interceptors)
}
