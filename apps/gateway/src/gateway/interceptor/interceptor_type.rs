use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InterceptorType {
    RequestId,
    RateLimiter,
}

impl InterceptorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterceptorType::RequestId => "request_id",
            InterceptorType::RateLimiter => "rate_limiter",
        }
    }
}
