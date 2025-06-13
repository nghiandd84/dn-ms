use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterceptorName {
    RequestId,
    RateLimiter,
}

impl InterceptorName {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterceptorName::RequestId => "request_id",
            InterceptorName::RateLimiter => "rate_limiter",
        }
    }
}
