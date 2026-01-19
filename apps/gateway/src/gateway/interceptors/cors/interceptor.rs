use async_trait::async_trait;
use tracing::debug;
use url::Url;

use crate::{
    config::proxy::http::Session,
    gateway::interceptor::{Interceptor, InterceptorType, Phase, PhaseMask, PhaseResult},
};

#[derive(Debug)]
pub struct CorsInterceptor {
    filter: Option<String>,
    domains: Vec<String>,
}

impl CorsInterceptor {
    pub fn build(filter: Option<String>, domains: Vec<String>) -> Self {
        Self { filter, domains }
    }

    fn extract_domain(url: Option<String>) -> Result<String, String> {
        match url {
            None => Err("URL is None".to_string()),
            Some(url) => {
                let parsed_url = Url::parse(&url).expect("Failed to parse URL");
                let host = parsed_url.host_str().expect("URL has no host");
                Ok(host.to_string())
            }
        }
    }
}

#[async_trait]
impl Interceptor for CorsInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::Cors
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::PostUpstreamResponse.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn post_upstream_response(&self, session: &mut Session) -> PhaseResult {
        let origin = session.get_req_header("Origin");
        debug!("CorsInterceptor Origin header: {:?}", origin);
        let origin_domain = match Self::extract_domain(origin) {
            Ok(domain) => domain,
            Err(_) => {
                debug!("CorsInterceptor: No valid Origin header found");
                return Ok(true);
            }
        };
        if !self.domains.is_empty() && !self.domains.contains(&origin_domain) {
            debug!(
                "CorsInterceptor: Origin domain {} not in allowed domains {:?}",
                origin_domain, self.domains
            );
            return Ok(true);
        }
        session.set_ds_res_header(
            "Access-Control-Allow-Origin".to_string(),
            origin_domain.into_bytes().to_vec(),
        );
        session.set_ds_res_header(
            "Access-Control-Allow-Methods".to_string(),
            "*".to_string().into_bytes().to_vec(),
        );
        session.set_ds_res_header(
            "Access-Control-Allow-Headers".to_string(),
            "*".to_string().into_bytes().to_vec(),
        );
        Ok(false)
    }
}
