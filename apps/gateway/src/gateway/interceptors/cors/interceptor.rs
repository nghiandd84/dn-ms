use async_trait::async_trait;
use http::{HeaderName, Method, StatusCode};
use pingora_http::ResponseHeader;
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

    fn extract_domain(url: String) -> Result<String, String> {
        let parsed_url = Url::parse(&url).expect("Failed to parse URL");
        let host = parsed_url.host_str().expect("URL has no host");
        Ok(host.to_string())
    }
}

#[async_trait]
impl Interceptor for CorsInterceptor {
    fn interceptor_type(&self) -> InterceptorType {
        InterceptorType::Cors
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        let origin = session.get_req_header("Origin");
        if origin.is_none() {
            debug!("CorsInterceptor: No Origin header present");
            return Ok(true);
        }
        let origin = origin.unwrap();
        debug!("CorsInterceptor Origin header: {:?}", origin);
        let origin_domain = match Self::extract_domain(origin.clone()) {
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
        let psession = session.get_psession();
        let header = psession.req_header();
        // Check prelight request
        if header.method == Method::OPTIONS {
            debug!("CorsInterceptor: Handling preflight OPTIONS request");
            let mut cors_resp = ResponseHeader::build(StatusCode::NO_CONTENT, None).unwrap();

            let _ = cors_resp.insert_header(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS, PUT, DELETE, PATCH",
            );
            let _ = cors_resp.insert_header("Access-Control-Allow-Headers", "*");
            let _ = cors_resp.insert_header("Access-Control-Allow-Credentials", "true");
            let _ = cors_resp.insert_header("Access-Control-Allow-Origin", origin);
            psession.set_keepalive(None);
            let _ = psession
                .write_response_header(Box::new(cors_resp), true)
                .await;
            return Ok(true);
        }
        session.set_ds_res_header(
            "Access-Control-Allow-Origin".to_string(),
            origin.into_bytes().to_vec(),
        );
        session.set_ds_res_header(
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PUT, DELETE, OPTIONS"
                .to_string()
                .into_bytes()
                .to_vec(),
        );
        session.set_ds_res_header(
            "Access-Control-Allow-Headers".to_string(),
            "*".to_string().into_bytes().to_vec(),
        );
        session.set_ds_res_header(
            "Access-Control-Allow-Credentials".to_string(),
            "true".to_string().into_bytes().to_vec(),
        );
        Ok(false)
    }
}
