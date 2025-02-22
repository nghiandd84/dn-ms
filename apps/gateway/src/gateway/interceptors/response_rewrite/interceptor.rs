use async_trait::async_trait;

use crate::{
    error::DakiaResult,
    gateway::interceptor::{Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult},
    proxy::http::Session,
};

use super::rewrite_parts::RewriteParts;

pub struct ResponseRewriteInterceptor {
    filter: Option<String>,
    rewrite_parts: RewriteParts,
}

impl ResponseRewriteInterceptor {
    pub fn build(filter: Option<String>, rewrite_parts: RewriteParts) -> Self {
        Self {
            filter,
            rewrite_parts,
        }
    }
}

#[async_trait]
impl Interceptor for ResponseRewriteInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::ResponseRewrite
    }

    fn phase_mask(&self) -> PhaseMask {
        Phase::Init.mask()
    }

    fn filter(&self) -> &Option<String> {
        &self.filter
    }

    async fn init(&self, _session: &mut Session) -> DakiaResult<()> {
        for (header_name, header_value) in &self.rewrite_parts.header_buffer {
            _session.set_ds_res_header(header_name.clone(), header_value.clone());
        }

        Ok(())
    }
}
