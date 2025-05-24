#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterceptorName {
    ServerVersion,
    UseFile,
    BasicAuth,
    Controller,
    RateLimiter,
    RequestRewrite,
    ResponseRewrite,
    ShortCircuit,
    RequestId,
}

impl InterceptorName {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterceptorName::ServerVersion => "server_version",
            InterceptorName::UseFile => "use_file",
            InterceptorName::BasicAuth => "basic_auth",
            InterceptorName::Controller => "controller",
            InterceptorName::RateLimiter => "rate_limiter",
            InterceptorName::RequestRewrite => "request_rewrite",
            InterceptorName::ResponseRewrite => "response_rewrite",
            InterceptorName::ShortCircuit => "short_circuit",
            InterceptorName::RequestId => "request_id",
        }
    }
}
