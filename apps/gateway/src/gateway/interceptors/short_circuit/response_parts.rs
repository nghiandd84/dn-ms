use std::collections::HashMap;

use http::StatusCode;

use crate::{
    config::source_config::InterceptorConfig,
    error::{DakiaError, DakiaResult},
    proxy::http::HeaderBuffer,
    qe::query::{extract_key_i64_or_err, extract_key_vec_bytes, Query},
};

pub struct ResponseParts {
    pub header_buffer: HeaderBuffer,
    pub status_code: Option<StatusCode>,
}

pub fn extract_headers(response_config: &Query) -> DakiaResult<HeaderBuffer> {
    let mut header_buf: HeaderBuffer = HashMap::new();

    for (header_key, _) in response_config {
        if header_key.starts_with("header.")
            || header_key.starts_with("res.header.")
            || header_key.starts_with("ds.res.header.")
        {
            // TODO: optimise this to only replace parts which is present
            let header_name = header_key
                .replace("ds.res.header.", "")
                .replace("res.header.", "")
                .replace("header.", "");

            let header_value = extract_key_vec_bytes(response_config, &header_key)?;
            header_buf.insert(header_name, header_value.unwrap_or(vec![]));
        }
    }

    Ok(header_buf)
}

impl ResponseParts {
    pub fn build(interceptor_config: &InterceptorConfig) -> DakiaResult<Self> {
        match &interceptor_config.response {
            Some(response) => {
                let header_buffer = extract_headers(response)?;
                let status_code = extract_key_i64_or_err(response, "status").unwrap_or(200);
                let status_code = StatusCode::from_u16(status_code as u16)?;

                Ok(Self {
                    header_buffer,
                    status_code: Some(status_code),
                })
            }
            None => Err(DakiaError::i_explain(format!(
                "response config is missing for {:?} interceptor",
                interceptor_config.name
            ))),
        }
    }
}
