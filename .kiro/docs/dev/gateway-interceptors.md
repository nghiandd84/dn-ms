# Gateway Interceptor System

The API gateway uses Pingora as a reverse proxy with a plugin-based interceptor system for cross-cutting concerns.

## Architecture

```
Client Request
  → Phase: RequestFilter
    → CorsInterceptor       (handle preflight, set CORS headers)
    → RateLimiterInterceptor (token bucket per client IP)
    → RequestIdInterceptor   (inject X-Request-Id)
    → TokenAuthInterceptor   (verify JWT, set baggage header)
  → Upstream Service
  → Phase: PostUpstreamResponse
    → (flush downstream response headers)
  → Client Response
```

### Location
- Interceptor trait: `apps/gateway/src/gateway/interceptor/interceptor.rs`
- Interceptor types: `apps/gateway/src/gateway/interceptor/interceptor_type.rs`
- Phase definitions: `apps/gateway/src/gateway/interceptor/phase.rs`
- Builder registry: `apps/gateway/src/gateway/interceptor_builder/mod.rs`
- Implementations: `apps/gateway/src/gateway/interceptors/{name}/`
- Config: `apps/gateway/config/config.yaml`

## Interceptor Trait

```rust
#[async_trait]
pub trait Interceptor: Send + Sync {
    fn interceptor_type(&self) -> InterceptorType;
    fn phase_mask(&self) -> PhaseMask;          // which phase to run in
    fn filter(&self) -> &Option<String>;         // path filter name
    async fn request_filter(&self, session: &mut Session) -> PhaseResult;
    // ... other phase hooks
}
```

**PhaseResult**: `Ok(false)` = continue chain, `Ok(true)` = short-circuit (stop processing).

## Available Interceptors

### RateLimiter (`rate_limiter`)
Token bucket algorithm with per-client-IP tracking using `DashMap`.

**Config:**
```yaml
- name: auth_rate_limiter
  type: rate_limiter
  enabled: true
  filter: auth_router_filter
  config:
    capacity: 5          # max burst tokens
    refill_rate: 2       # tokens added per interval
    refill_interval: 10  # seconds between refills
```

**Behavior:**
- Tracks tokens per client IP address
- On each request: refills tokens based on elapsed time, then consumes one
- If no tokens available: returns `429 Too Many Requests` with `Retry-After` header
- In-memory state (resets on gateway restart)

### TokenAuth (`token_auth`)
Verifies JWT tokens via the auth service and injects baggage header for downstream services.

**Config:**
```yaml
- name: auth_token_auth
  type: token_auth
  enabled: true
  filter: auth_router_filter
  config:
    use_auth_service: true
```

**Behavior:**
- Skips paths starting with `/public/`
- Extracts `Authorization: Bearer <token>` header
- Calls auth service to verify token
- On success: converts access token to baggage header for upstream
- On failure: short-circuits with `Ok(true)`

### CORS (`cors`)
Handles CORS preflight and response headers.

**Config:**
```yaml
- name: bakery_cors
  type: cors
  enabled: true
  filter: bakery_router_filter
  config:
    allowed_domains: mydomain.com,testdomain.com
```

**Behavior:**
- Checks `Origin` header against allowed domains
- OPTIONS preflight: returns `204 No Content` with CORS headers
- Other methods: sets CORS response headers for downstream flush

### RequestId (`request_id`)
Injects a unique `X-Request-Id` header into upstream requests.

**Config:**
```yaml
- name: auth_request_id
  type: request_id
  enabled: true
  filter: auth_router_filter
```

## Adding a New Interceptor

### 1. Create interceptor module
```
apps/gateway/src/gateway/interceptors/{name}/
├── mod.rs           # pub use builder + interceptor
├── interceptor.rs   # impl Interceptor
└── builder.rs       # impl InterceptorBuilder
```

### 2. Implement the interceptor
```rust
#[async_trait]
impl Interceptor for MyInterceptor {
    fn interceptor_type(&self) -> InterceptorType { InterceptorType::MyType }
    fn phase_mask(&self) -> PhaseMask { Phase::RequestFilter.mask() }
    fn filter(&self) -> &Option<String> { &self.filter }

    async fn request_filter(&self, session: &mut Session) -> PhaseResult {
        // Ok(false) = pass, Ok(true) = short-circuit
    }
}
```

### 3. Implement the builder
```rust
impl InterceptorBuilder for MyInterceptorBuilder {
    fn build(&self, config: InterceptorConfig) -> GatewayResult<Arc<dyn Interceptor>> {
        // parse config.config HashMap<String, String>
        Ok(Arc::new(MyInterceptor::build(...)))
    }
}
```

### 4. Register in InterceptorType enum
Add variant to `interceptor_type.rs` and register builder in `InterceptorBuilderRegistry::build()`.

### 5. Add to config.yaml
```yaml
interceptors:
  - name: service_my_interceptor
    type: my_type
    enabled: true
    filter: service_router_filter
    config:
      key: value
```

## Config Structure

Each service in `config.yaml` should have these interceptors (in order):
1. `request_id` — trace correlation
2. `token_auth` — authentication
3. `cors` — cross-origin support
4. `rate_limiter` — traffic protection

Interceptors are executed in the order they appear in the config. Each is scoped to a `filter` that matches request paths.

## Session API

Key methods available in interceptors:

| Method | Description |
|---|---|
| `session.get_req_header("Name")` | Read downstream request header |
| `session.ds_req_header("Name")` | Read downstream request header (alias) |
| `session.ds_req_path()` | Get request path |
| `session.set_us_req_header(name, value)` | Set header on upstream request |
| `session.set_ds_res_header(name, value)` | Set header on downstream response |
| `session.get_psession()` | Access raw Pingora session |
| `session.get_span_context()` | Get OpenTelemetry span context |
| `session.set_span_context(ctx)` | Set OpenTelemetry span context |

To write a direct response (e.g., 429, 204):
```rust
let psession = session.get_psession();
let mut resp = ResponseHeader::build(StatusCode::TOO_MANY_REQUESTS, None).unwrap();
resp.insert_header("Retry-After", "10");
psession.set_keepalive(None);
psession.write_response_header(Box::new(resp), false).await;
psession.write_response_body(Some(Bytes::from("Too Many Requests")), true).await;
return Ok(true); // short-circuit
```
