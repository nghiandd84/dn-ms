# URL Shortener Service API

Manages URL shortening, redirect resolution, click analytics, and API key management.

## Location

- API crate: `apis/url-shortener/`
- Feature crates: `features/url-shortener/{entities,model,repo,service,migrations}`

## Endpoints

### Shortened URLs
- `POST /urls` — Create shortened URL (Auth: `CanCreateUrl`)
- `GET /urls` — List user's URLs with pagination/filters (Auth: `CanCreateUrl`)
- `GET /urls/{id}` — Get URL details by ID (Auth: `CanCreateUrl`)
- `PATCH /urls/{id}` — Update URL (title, is_active, expires_at) (Auth: `CanUpdateUrl`)
- `DELETE /urls/{id}` — Delete URL (Auth: `CanDeleteUrl`)

### Redirect
- `GET /r/{code}` — Public redirect endpoint. Returns 302 on success, HTML error page on expired/inactive links.

### Click Analytics
- `GET /urls/{id}/clicks` — Get paginated click records for a URL (Auth: `CanViewAnalytics`)

### API Keys
- `POST /api-keys` — Create new API key (Auth: `CanManageApiKeys`)
- `GET /api-keys` — List user's API keys (Auth: `CanManageApiKeys`)
- `DELETE /api-keys/{id}` — Revoke an API key (Auth: `CanDeleteApiKey`)

## Request/Response Examples

### Create Shortened URL
```json
POST /urls
{
  "original_url": "https://example.com/very/long/path",
  "custom_code": "my-brand",    // optional, 3-30 chars
  "title": "My Link",           // optional
  "expires_at": "2026-12-31T23:59:59"  // optional
}
```

Response:
```json
{
  "ok": true,
  "id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Create API Key
```json
POST /api-keys
{
  "name": "My Integration Key"
}
```

Response (key shown only once):
```json
{
  "id": "...",
  "name": "My Integration Key",
  "key": "a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1",
  "created_at": "2026-07-15T04:00:00"
}
```

## Permissions

| Permission | Resource | Action |
|-----------|----------|--------|
| CanCreateUrl | URL_SHORTENER:URL | CREATE |
| CanUpdateUrl | URL_SHORTENER:URL | UPDATE |
| CanDeleteUrl | URL_SHORTENER:URL | DELETE |
| CanViewAnalytics | URL_SHORTENER:ANALYTICS | READ |
| CanManageApiKeys | URL_SHORTENER:API_KEY | CREATE |
| CanDeleteApiKey | URL_SHORTENER:API_KEY | DELETE |

## Caching Strategy

- **Backend**: Redis via `shared-shared-data-cache`
- **Key pattern**: `url_shortener:{short_code}`
- **TTL**: 5 minutes
- **Cached value**: `{url_id, original_url, expires_at, is_active}`
- **Invalidation**: On update or delete of a shortened URL

## Custom Metrics (OpenTelemetry)

| Metric | Type | Description |
|--------|------|-------------|
| `url_shortener.redirects_total` | Counter | Total redirects (labels: status=success/expired/inactive) |
| `url_shortener.urls_created_total` | Counter | Total URLs created (labels: code_type=auto/custom) |
| `url_shortener.expired_redirects_total` | Counter | Expired/inactive redirect attempts |
| `url_shortener.redirect_latency` | Histogram | Redirect resolution time (seconds) |
| `url_shortener.cache_hits_total` | Counter | Redis cache hits |
| `url_shortener.cache_misses_total` | Counter | Redis cache misses |

## Configuration (.env)

```
URL_SHORTENER_REDIS_URL=redis://:Redis!123@localhost:6379
URL_SHORTENER_DATABASE_READ_URL=${DATABASE_URL}
URL_SHORTENER_DATABASE_WRITE_URL=${DATABASE_URL}
URL_SHORTENER_DATABASE_SCHEME=url_shortener
URL_SHORTENER_PORT=5201
```

## Short Code Generation

- **Charset**: `[a-zA-Z0-9]` (base62, 62 characters)
- **Length**: 7 characters (~3.5 trillion combinations)
- **Collision handling**: Retry up to 3 times on collision
- **Custom codes**: 3-30 characters, uniqueness validated before creation
