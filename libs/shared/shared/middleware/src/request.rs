use http::{Extensions, HeaderName, HeaderValue};
use opentelemetry::global;
use opentelemetry::propagation::Injector;
use reqwest::{header::HeaderMap, Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use tracing::debug;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct RequestTracingMiddleware;

#[async_trait::async_trait]
impl Middleware for RequestTracingMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // 1. Get the current tracing span context
        let context = tracing::Span::current().context();
        // 2. Inject the context into the request headers
        // This uses the global propagator (usually W3C TraceContext by default)
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut HeaderInjector(req.headers_mut()));
        });
        debug!("Request started {:?}", req);
        let res = next.run(req, extensions).await;
        debug!("Result: {:?}", res);
        res
    }
}

// Helper to allow OTEL to write into Reqwest headers
struct HeaderInjector<'a>(&'a mut HeaderMap);

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_bytes(key.as_bytes()),
            HeaderValue::from_str(&value),
        ) {
            self.0.insert(name, val);
        }
    }
}
