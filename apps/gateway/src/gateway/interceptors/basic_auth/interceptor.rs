use std::str::from_utf8;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use http::StatusCode;

use crate::{
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

pub struct BasicAuthInterceptor {
    credentials: String,
    filter: Option<String>,
}

impl BasicAuthInterceptor {
    pub fn build(filter: Option<String>, username: String, password: String) -> Self {
        let user_pass = format!("{}:{}", username, password);
        let credentials = general_purpose::STANDARD.encode(user_pass.clone());

        BasicAuthInterceptor {
            filter,
            credentials,
        }
    }

    fn authorize(&self, auth_header_bytes: &[u8]) -> bool {
        match from_utf8(auth_header_bytes) {
            Ok(user_credentials) => match user_credentials.strip_prefix("Basic ") {
                Some(stripped) => self.credentials == stripped,
                None => false,
            },
            Err(_) => false, // Authorization header must be an valid UTF-8 string
        }
    }
}

#[async_trait]
impl Interceptor for BasicAuthInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::BasicAuth
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::RequestFilter.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        let auth_header_option = _session.ds_req_header("Authorization")?;
        /*
        TODO: Move code to write unauthorized response to a common method

        Can not right now because of the following error
        - implicit elided lifetime not allowed here expected lifetime parameter
        */

        match auth_header_option {
            Some(auth_header_bytes) => {
                let is_authorized = self.authorize(auth_header_bytes);

                if is_authorized {
                    Ok(false)
                } else {
                    _session.set_res_status(StatusCode::UNAUTHORIZED);
                    _session.set_ds_res_header(
                        "WWW-Authenticate".to_string(),
                        "Basic realm=\"Protected Area\"".as_bytes().to_vec(),
                    );
                    Ok(true)
                }
            }
            None => {
                _session.set_res_status(StatusCode::UNAUTHORIZED);
                _session.set_ds_res_header(
                    "WWW-Authenticate".to_string(),
                    "Basic realm=\"Protected Area\"".as_bytes().to_vec(),
                );
                Ok(true)
            }
        }
    }
}
