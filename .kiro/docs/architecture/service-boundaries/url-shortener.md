# URL Shortener Service

Provides URL shortening, redirect resolution with caching, click analytics, and API key management.

## Responsibilities
- Create shortened URLs with auto-generated or custom short codes
- Resolve short codes to original URLs with 302 redirect
- Redis caching of redirect lookups (5-min TTL) with invalidation on mutations
- Track click analytics (IP, user-agent, referrer, country, timestamp)
- API key management (create, validate, revoke) using SHA-256 hashing
- Custom OpenTelemetry metrics for monitoring redirect performance and cache efficiency
- Return styled HTML error page for expired or inactive links

## Crate Structure
| Crate | Path | Purpose |
|-------|------|---------|
| features-url-shortener-entities | `features/url-shortener/entities` | SeaORM entities (shortened_url, url_click, api_key) |
| features-url-shortener-model | `features/url-shortener/model` | DTOs, request/response types, validators, cache struct, state |
| features-url-shortener-repo | `features/url-shortener/repo` | Query and Mutation repos using derive macros |
| features-url-shortener-service | `features/url-shortener/service` | Business logic, Redis cache, metrics, code generation |
| features-url-shortener-migrations | `features/url-shortener/migrations` | SeaORM migrations for schema creation |
| api-url-shortener | `apis/url-shortener` | Axum API routes, permissions, Swagger UI, error pages |

## Key Patterns
- **Redis cache-first reads**: Redirect lookups check Redis before DB, cache on miss
- **Fire-and-forget click recording**: Click analytics are recorded asynchronously via `tokio::spawn`
- **Ownership verification**: Update/delete operations verify `user_id` matches before proceeding
- **API key hashing**: Keys stored as SHA-256 hash; plaintext shown only once on creation
- **Custom OTel metrics**: `once_cell::Lazy` global metrics struct with counters and histograms

## Database Schema

### shortened_urls
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK, auto-generated |
| user_id | UUID | NOT NULL, indexed |
| original_url | TEXT | NOT NULL |
| short_code | VARCHAR(30) | UNIQUE, NOT NULL |
| title | VARCHAR(255) | nullable |
| is_active | BOOLEAN | DEFAULT true |
| expires_at | TIMESTAMP | nullable |
| click_count | BIGINT | DEFAULT 0 |
| created_at | TIMESTAMP | NOT NULL |
| updated_at | TIMESTAMP | NOT NULL |

### url_clicks
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| url_id | UUID | FK → shortened_urls.id (CASCADE) |
| ip_address | VARCHAR(45) | nullable |
| user_agent | TEXT | nullable |
| referrer | TEXT | nullable |
| country | VARCHAR(2) | nullable |
| clicked_at | TIMESTAMP | NOT NULL |
| created_at | TIMESTAMP | NOT NULL |

### api_keys
| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | PK |
| user_id | UUID | NOT NULL, indexed |
| key_hash | VARCHAR(64) | UNIQUE, NOT NULL |
| name | VARCHAR(100) | NOT NULL |
| is_active | BOOLEAN | DEFAULT true |
| last_used_at | TIMESTAMP | nullable |
| created_at | TIMESTAMP | NOT NULL |

## Consumed By
- Frontend web app — for creating/managing short URLs and viewing analytics
- External integrations — via API keys for programmatic URL shortening
- Gateway — routes `/api/url-shortener` prefix and public `/r/{code}` endpoint

## Dependencies
- `features-auth-remote` — Permission system integration (RBAC)
- `shared-shared-data-cache` — Redis caching
- `shared-shared-observability` — OpenTelemetry metrics export
