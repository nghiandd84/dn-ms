# Anomaly Detection Interceptor

The gateway includes an `anomaly_detector` interceptor that detects and blocks abnormal requests using Redis-backed stateful tracking.

## Architecture

```
Client Request
  → AnomalyDetectorInterceptor (RequestFilter phase)
    → extract_client_identity()
    → Run detection rules sequentially:
        1. PayloadSizeRule
        2. DuplicatePayloadRule
        3. EndpointScanRule (check if IP blocked)
        4. AuthBruteForceRule (check if IP blocked)
        5. RapidPathSwitchRule
    → If any rule triggers: log + emit metric + block
    → Otherwise: pass through

  → AnomalyDetectorInterceptor (PostUpstreamResponse phase)
    → EndpointScanRule: track 404 responses
    → AuthBruteForceRule: track 401/403 on auth paths
```

### Location

```
apps/gateway/src/gateway/interceptors/anomaly_detector/
├── mod.rs              # Module exports
├── interceptor.rs      # Main interceptor logic + client identity extraction
├── builder.rs          # Config parsing + rule wiring
└── rules/
    ├── mod.rs              # DetectionRule trait, RequestContext, Violation
    ├── payload_size.rs     # Oversized payload detection
    ├── duplicate_payload.rs # Repeated payload detection (same client + multi-IP)
    ├── endpoint_scan.rs    # 404 scanning detection
    ├── auth_brute_force.rs # Auth brute force detection
    └── rapid_path_switch.rs # Reconnaissance pattern detection
```

## Client Identity Strategy

The interceptor uses a priority-based strategy to identify clients, solving the shared-IP problem (proxies, NATs, corporate networks):

| Priority | Source | Key Prefix | Description |
|----------|--------|------------|-------------|
| 1 | `X-Client-Fingerprint` header | `cfp:` | Browser-side fingerprint (FingerprintJS). Validated as hex, min 16 chars. |
| 2 | `Authorization: Bearer <token>` | `user:` | SHA-256 hash of the JWT token. |
| 3 | IP + User-Agent + Accept-Language | `fp:` | Server-side composite fingerprint. |
| 4 | Client IP address | `ip:` | Last resort fallback. |

### Frontend Integration

The frontend should compute a browser fingerprint and send it as a header:

```javascript
import FingerprintJS from '@fingerprintjs/fingerprintjs';

const fp = await FingerprintJS.load();
const result = await fp.get();

// Attach to every request
headers['X-Client-Fingerprint'] = result.visitorId;
```

## Detection Rules

### PayloadSizeRule

Blocks requests with `Content-Length` exceeding the configured maximum.

- **Response:** `413 Payload Too Large`
- **Config:** `max_payload_size` (bytes, default: 1048576 = 1MB)
- **Stateless:** No Redis needed

### DuplicatePayloadRule

Detects repeated identical payloads using SHA-256 hash of `method + path + content_length`.

- **Response:** `429 Too Many Requests`
- **Same-client detection:** Blocks when same client sends identical payload more than `duplicate_threshold` times within `duplicate_window`
- **Multi-IP detection:** Blocks when the same payload arrives from more than `duplicate_multi_ip_threshold` distinct clients
- **Redis keys:**
  - `anomaly:{client_id}:payload:{hash}` — counter with TTL
  - `anomaly:payload:{hash}:ips` — comma-separated IP list with TTL

### EndpointScanRule

Detects clients hitting many non-existent endpoints (404 responses).

- **Response:** `403 Forbidden`
- **Tracking:** Counts 404 responses in `PostUpstreamResponse` phase
- **Blocking:** Once `max_404_count` exceeded, IP is flagged for `block_duration`
- **Redis keys:**
  - `anomaly:{client_id}:not_found` — counter with TTL
  - `anomaly:{client_id}:blocked:scan` — block flag with TTL

### AuthBruteForceRule

Detects repeated failed authentication attempts on auth-related paths.

- **Response:** `403 Forbidden` with `Retry-After` header
- **Tracking:** Counts 401/403 responses on paths containing `/auth`
- **Blocking:** Once `max_auth_failures` exceeded, IP is blocked for `block_duration`
- **Redis keys:**
  - `anomaly:{client_id}:auth_failures` — counter with TTL
  - `anomaly:{client_id}:blocked:auth` — block flag with TTL

### RapidPathSwitchRule

Detects clients accessing many distinct endpoints unusually fast (reconnaissance).

- **Response:** `403 Forbidden`
- **Tracking:** Maintains list of distinct paths accessed within `path_window`
- **Blocking:** Once `max_distinct_paths` exceeded, IP is blocked for `block_duration`
- **Redis keys:**
  - `anomaly:{client_id}:paths` — newline-separated path list with TTL
  - `anomaly:{client_id}:blocked:paths` — block flag with TTL

## Configuration

Config is provided via `config.yaml` in the interceptors section:

```yaml
# Global anomaly detector (applies to all routes when no filter specified)
- name: global_anomaly_detector
  type: anomaly_detector
  enabled: true
  config:
    redis_url: redis://127.0.0.1/
    max_payload_size: "1048576"        # 1MB
    duplicate_threshold: "5"           # same payload from same client
    duplicate_multi_ip_threshold: "10" # same payload from N distinct clients
    duplicate_window: "60"             # seconds
    max_404_count: "10"                # 404s before blocking
    not_found_window: "60"             # seconds
    max_auth_failures: "5"             # failed auths before blocking
    auth_window: "300"                 # seconds
    block_duration: "600"              # how long to block (seconds)
    max_distinct_paths: "50"           # paths before flagging as scan
    path_window: "60"                  # seconds

# Per-filter override (stricter for auth routes)
- name: auth_anomaly_detector
  type: anomaly_detector
  enabled: true
  filter: auth_router_filter
  config:
    redis_url: redis://127.0.0.1/
    max_payload_size: "524288"
    duplicate_threshold: "3"
    max_auth_failures: "3"
    block_duration: "900"
```

### Config Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `redis_url` | `redis://127.0.0.1/` | Redis connection URL |
| `max_payload_size` | `1048576` | Max allowed Content-Length in bytes |
| `duplicate_threshold` | `5` | Max identical payloads from same client |
| `duplicate_multi_ip_threshold` | `10` | Max distinct IPs sending same payload |
| `duplicate_window` | `60` | Time window for duplicate detection (seconds) |
| `max_404_count` | `10` | Max 404 responses before blocking |
| `not_found_window` | `60` | Time window for 404 counting (seconds) |
| `max_auth_failures` | `5` | Max auth failures before blocking |
| `auth_window` | `300` | Time window for auth failure counting (seconds) |
| `block_duration` | `600` | How long a blocked client stays blocked (seconds) |
| `max_distinct_paths` | `50` | Max distinct paths before flagging as scan |
| `path_window` | `60` | Time window for path counting (seconds) |

## Observability

### Structured Logging

When an anomaly is detected, a `tracing::warn!` is emitted with:

```
client_id, rule_name, violation_type, path, action, reason
```

### OpenTelemetry Metrics

- **Meter:** `gateway_anomaly_detector`
- **Counter:** `gateway.anomaly.blocked` with label `rule` (e.g., `payload_size`, `duplicate_payload`, `endpoint_scan`, `auth_brute_force`, `rapid_path_switch`)

## Adding a New Detection Rule

1. Create a new file in `rules/` implementing the `DetectionRule` trait:

```rust
#[async_trait]
pub trait DetectionRule: Send + Sync {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation>;
    async fn post_response(&self, _ctx: &RequestContext, _session: &mut Session, _cache: &Cache<String, String>) {}
    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult;
}
```

2. Add the module to `rules/mod.rs`
3. Instantiate and add to the `rules` vec in `builder.rs`

## Future Detection Rules (Ideas for Implementation)

### Challenge-Based (soft block)

| Rule | Response | Description |
|------|----------|-------------|
| **CaptchaChallengeRule** | `200` with challenge page | For medium-risk requests, serve a CAPTCHA instead of hard-blocking. Require the client to solve and retry with a proof token. |
| **JsChallengeRule** | `200` with JS proof-of-work | Cloudflare-style: serve a JavaScript challenge that requires browser computation. Block clients that can't execute JS (bots). |
| **EmailOtpVerificationRule** | `200` with OTP form | After detected suspicious login, require additional email verification before proceeding. |

### Behavioral & Analysis

| Rule | Response | Description |
|------|----------|-------------|
| **UserRateLimiterRule** | `429` | Token bucket per authenticated `user_id` (not just IP). Track via `user:{id}:rate`. |
| **DeviceFingerprintCorrelationRule** | `403` | Cross-check inconsistency between fingerprint signals — e.g., screen_resolution says iPhone but user_agent says Linux, or Accept-Language mismatch with IP geo. |
| **VelocityCheckRule** | `429` / `403` | Track operation speed per-client: e.g., 50 items added to cart in 1 second, or 10 concurrent logins from different IPs with the same credentials. Redis: `anomaly:{client_id}:velocity:{operation}` counter. |
| **TemporalAnomalyRule** | `403` | Learn user's typical access hours (stored in Redis with decay). Flag requests at hours the user has never accessed before. |

### WAF-Style

| Rule | Response | Description |
|------|----------|-------------|
| **InputValidationRule** | `400` | Block SQL injection patterns (`' OR 1=1--`), XSS payloads (`<script>`), path traversal (`../`). Stateless regex check on query params and body. |
| **HoneypotRule** | `403` + block | Define hidden endpoint paths (e.g., `/admin`, `/wp-admin`, `/.env`). Any request hitting these is automatically blocked and the client fingerprint is banned. |
| **HeaderValidationRule** | `400` | Reject requests with inconsistent/missing headers — e.g., a browser claiming Chrome but missing `Sec-CH-UA`, or no `Accept-Language` but present `Referer`. |

### External Intelligence

| Rule | Response | Description |
|------|----------|-------------|
| **IpReputationRule** | `403` | Check client IP against a threat intelligence feed (e.g., AbuseIPDB) or known VPN/Tor exit node list. Cache results in Redis with TTL. |
| **GeoblockingRule** | `403` | Allow/block traffic by geo region (MaxMind GeoIP). Per-service configurable in `config.yaml`. |

### Account Security

| Rule | Response | Description |
|------|----------|-------------|
| **NewDeviceDetectionRule** | Challenge (step-up auth) | Track known device fingerprints per user. When a login occurs from an unseen fingerprint, require additional verification before issuing the token. |
| **CredentialStuffingRule** | Rate limit + block | Track failed login attempts per-IP per-credential hash (not just per-IP). Block when the same credential is tried from many IPs. |

### Implementation Pattern

All rules follow the same `DetectionRule` trait:

```rust
#[async_trait]
pub trait DetectionRule: Send + Sync {
    async fn check(&self, ctx: &RequestContext, cache: &Cache<String, String>) -> Option<Violation>;
    async fn post_response(&self, _ctx: &RequestContext, _session: &mut Session, _cache: &Cache<String, String>) {}
    async fn respond(&self, session: &mut Session, violation: &Violation) -> PhaseResult;
}
```

Steps to implement any of the above:
1. Create `apps/gateway/src/gateway/interceptors/anomaly_detector/rules/<name>.rs`
2. Implement `DetectionRule` for your struct
3. Register in `rules/mod.rs`
4. Add config fields to `builder.rs`
5. Add config parameters to the YAML under `apps/gateway/config/config.yaml`
