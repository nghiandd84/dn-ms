use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use tracing::debug;

use crate::{
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

pub struct UseFileInterceptor {
    root: String,
    filter: Option<String>,
}

impl UseFileInterceptor {
    pub fn build(root: String, filter: Option<String>) -> Self {
        UseFileInterceptor { root, filter }
    }
}

#[async_trait]
impl Interceptor for UseFileInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::UseFile
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::UpstreamProxyFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        let path = _session.ds_req_path();
        let aboslute_path = format!("{}{}", self.root, path);

        match tokio::fs::read(aboslute_path.clone()).await {
            Ok(file_content) => {
                _session.set_ds_res_header(
                    "Content-Length".to_string(),
                    file_content.len().to_string().as_bytes().to_vec(),
                );

                _session
                    .write_ds_res_body(Some(Bytes::from(file_content)), true)
                    .await?;
            }
            Err(err) => {
                debug!("can not read file {aboslute_path} - {err}");
                _session.set_res_status(StatusCode::NOT_FOUND);
            }
        };

        Ok(true)
    }
}
