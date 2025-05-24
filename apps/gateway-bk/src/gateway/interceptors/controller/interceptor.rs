use std::str::from_utf8;

use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;

use crate::{
    config::{source_config::SourceDakiaRawConfig, DakiaConfig},
    error::DakiaResult,
    gateway::{
        interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
        state::build_gateway_state,
    },
    proxy::http::Session,
    shared::dakia_state::DAKIA_STATE_STORE,
};

pub struct ControllerInterceptor {
    filter: Option<String>,
}

impl ControllerInterceptor {
    pub fn build(filter: Option<String>) -> Self {
        Self { filter }
    }

    async fn write_invalid_method_response(&self, _session: &mut Session<'_>) -> DakiaResult<()> {
        _session.set_res_status(StatusCode::METHOD_NOT_ALLOWED);
        Ok(())
    }

    async fn write_bad_request_response(&self, _session: &mut Session<'_>) -> DakiaResult<()> {
        _session.set_res_status(StatusCode::BAD_REQUEST);
        Ok(())
    }

    async fn write_invalid_content_type_response(
        &self,
        _session: &mut Session<'_>,
    ) -> DakiaResult<()> {
        _session.set_res_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
        Ok(())
    }

    async fn store_dakia_config_in_store(&self, mut dakia_config: DakiaConfig) -> DakiaResult<()> {
        let cur_dakia_config = DAKIA_STATE_STORE.get_dakia_config().unwrap();
        dakia_config.version = cur_dakia_config.version + 1;

        for gateway_config in &dakia_config.gateways {
            let gateway_state =
                build_gateway_state(gateway_config.clone(), dakia_config.version).await?;
            DAKIA_STATE_STORE.update_gateway_state(gateway_state)?;
        }

        DAKIA_STATE_STORE.store_dakia_config(dakia_config)?;
        Ok(())
    }

    async fn update_in_memory_dakia_config(&self, _session: &mut Session<'_>) -> DakiaResult<()> {
        let body = _session.read_ds_req_body().await?;
        let body_str = match &body {
            Some(bval) => {
                if bval.is_empty() {
                    return self.write_bad_request_response(_session).await;
                } else {
                    from_utf8(&bval).expect("Failed to parse content: invalid UTF-8 encoding")
                }
            }
            None => return self.write_bad_request_response(_session).await,
        };

        let content_type_hedaer = _session.ds_req_header("Content-Type")?;
        match content_type_hedaer {
            Some(havl) => {
                let source_dakia_raw_config = if havl == "application/json".as_bytes() {
                    let source_dakia_config: SourceDakiaRawConfig = serde_json::from_str(body_str)
                        .expect("Failed to deserialize: invalid json body");
                    source_dakia_config
                } else if havl == "application/yaml".as_bytes() {
                    let source_dakia_config: SourceDakiaRawConfig = serde_yaml::from_str(body_str)
                        .expect("Failed to deserialize: invalid yaml body");
                    source_dakia_config
                } else {
                    return self.write_invalid_content_type_response(_session).await;
                };

                self.store_dakia_config_in_store(DakiaConfig::from(source_dakia_raw_config))
                    .await?;
                Ok(())
            }
            None => return self.write_invalid_content_type_response(_session).await,
        }
    }

    async fn write_invalid_accept_header_response(
        &self,
        _session: &mut Session<'_>,
    ) -> DakiaResult<()> {
        _session.set_res_status(StatusCode::NOT_ACCEPTABLE);
        Ok(())
    }

    async fn write_dakia_config_in_response(&self, _session: &mut Session<'_>) -> DakiaResult<()> {
        let dakia_config = DAKIA_STATE_STORE.get_dakia_config()?;
        let source_dakia_raw_config = SourceDakiaRawConfig::from(dakia_config);
        let accept_header = _session.ds_req_header("Accept")?;

        match accept_header {
            Some(hval) => {
                let config_str = if hval == "application/json".as_bytes() {
                    serde_json::to_string(&source_dakia_raw_config)
                        .expect("Can not serialize config to json")
                }
                // https://www.ietf.org/archive/id/draft-ietf-httpapi-yaml-mediatypes-00.html#name-media-type-application-yaml
                else if hval == "application/yaml".as_bytes() {
                    serde_yaml::to_string(&source_dakia_raw_config)
                        .expect("Can not serialize config to json")
                } else {
                    self.write_invalid_accept_header_response(_session).await?;
                    return Ok(());
                };

                _session
                    .write_ds_res_body(Some(Bytes::from(config_str)), true)
                    .await?;
            }
            None => return self.write_invalid_accept_header_response(_session).await,
        }

        Ok(())
    }
}

#[async_trait]
impl Interceptor for ControllerInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::Controller
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::UpstreamProxyFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        let method = _session.ds_req_method()?;
        if method == "GET" {
            self.write_dakia_config_in_response(_session).await?;
        } else if method == "PUT" {
            self.update_in_memory_dakia_config(_session).await?;
        } else {
            self.write_invalid_method_response(_session).await?;
        }

        Ok(true)
    }
}
