use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::{DakiaError, DakiaResult},
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::basic_auth::BasicAuthInterceptor,
    },
    qe::query::{extract_key_str_or_err, Query},
};

pub struct BasicAuthInterceptorBuilder {}

impl BasicAuthInterceptorBuilder {
    fn get_user_pass(config: &Query) -> DakiaResult<(String, String)> {
        let username = extract_key_str_or_err(config, "username")?;
        let password = extract_key_str_or_err(config, "password")?;
        Ok((username.to_string(), password.to_string()))
    }
}
impl Default for BasicAuthInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for BasicAuthInterceptorBuilder {
    fn build(&self, _interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        match &_interceptor_config.config {
            Some(config) => {
                let (username, password) = BasicAuthInterceptorBuilder::get_user_pass(config)?;
                let interceptor =
                    BasicAuthInterceptor::build(_interceptor_config.filter, username, password);
                Ok(Arc::new(interceptor))
            }
            None => Err(DakiaError::i_explain(format!(
                "config required for interceptor {:?}",
                _interceptor_config.name
            ))),
        }
    }
}
