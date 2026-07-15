# URL Shortener Database Schema

Database: PostgreSQL  
Schema: `url_shortener`

## Tables

### shortened_urls

Primary table storing all shortened URL mappings.

```sql
CREATE TABLE shortened_urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    original_url TEXT NOT NULL,
    short_code VARCHAR(30) NOT NULL UNIQUE,
    title VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMP,
    click_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_shortened_urls_user_id ON shortened_urls(user_id);
```

### url_clicks

Click analytics table — one row per redirect event.

```sql
CREATE TABLE url_clicks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url_id UUID NOT NULL REFERENCES shortened_urls(id) ON DELETE CASCADE,
    ip_address VARCHAR(45),
    user_agent TEXT,
    referrer TEXT,
    country VARCHAR(2),
    clicked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_url_clicks_url_id_clicked_at ON url_clicks(url_id, clicked_at);
```

### api_keys

API keys for programmatic access. Stores SHA-256 hash of the key.

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_used_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
```

## Entity Relationships

```
shortened_urls 1──∞ url_clicks (url_id → id, CASCADE delete)
```

## Migration

Migration file: `features/url-shortener/migrations/src/m20260715_000001_create_url_shortener_tables.rs`

Run migration:
```bash
cargo run --bin migrations_url_shortener -- -u postgresql://dn_ms:password123@127.0.0.1:5432/dn_ms -s url_shortener up
```
