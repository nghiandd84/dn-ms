use std::collections::HashMap;

use crate::{
    config::source_config::InterceptorConfig,
    error::{DakiaError, DakiaResult},
    proxy::http::HeaderBuffer,
    qe::query::{extract_key_vec_bytes, Query},
};

pub struct RewriteParts {
    pub header_buffer: HeaderBuffer,
}

pub fn extract_headers(rewrite_config: &Query) -> DakiaResult<HeaderBuffer> {
    let mut header_buf: HeaderBuffer = HashMap::new();

    for (header_key, _) in rewrite_config {
        if header_key.starts_with("header.")
            || header_key.starts_with("res.header.")
            || header_key.starts_with("ds.res.header.")
        {
            // TODO: optimise this to only replace parts which is present
            let header_name = header_key
                .replace("ds.res.header.", "")
                .replace("res.header.", "")
                .replace("header.", "");

            let header_value = extract_key_vec_bytes(rewrite_config, &header_key)?;
            header_buf.insert(header_name, header_value.unwrap_or(vec![]));
        }
    }

    Ok(header_buf)
}

impl RewriteParts {
    pub fn build(interceptor_config: &InterceptorConfig) -> DakiaResult<Self> {
        match &interceptor_config.rewrite {
            Some(rewrite) => {
                let header_buffer = extract_headers(rewrite)?;

                Ok(Self { header_buffer })
            }
            None => Err(DakiaError::i_explain(format!(
                "rewrite config is missing for {:?} interceptor",
                interceptor_config.name
            ))),
        }
    }
}
