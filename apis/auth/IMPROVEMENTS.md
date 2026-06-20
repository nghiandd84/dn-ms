# Auth API - Improvement Suggestions

## Critical (Security/Stability)

### ~~1. `.unwrap()` on refresh token decode~~ ✅ DONE
The refresh token flow panics if a malformed token is submitted, crashing the server.
**Fix:** Replaced `.unwrap()` with `.map_err(|e| AppError::Token(e))?` for proper error propagation.

### ~~2. Token verify doesn't check JTI cache~~ ✅ DONE
A revoked token remains valid until expiry because the verify endpoint doesn't check if the JTI still exists in cache.
**Fix:** Added `decode_access_token_with_jti` to shared-auth; `verify_token` now checks cache for matching JTI before returning success.

### ~~3. Auth codes not expiry-checked~~ ✅ DONE
`expires_at` is stored but never filtered in queries. An expired code can still be exchanged for tokens.
**Fix:** Added `expires_at > now()` filter to `get_by_client_id_and_code` query.

### 4. No rate limiting on public endpoints
Login, register, and token endpoints are open to brute-force and credential stuffing.
**Fix:** Add rate limiting middleware (e.g., tower-governor or custom token bucket) on `/public/*` routes.

### ~~5. Client secrets exposed in API responses~~ ✅ DONE
`ClientData` serializes `client_secret` in GET responses.
**Fix:** Added `#[serde(skip_serializing)]` to `client_secret` field in `ClientData` — never appears in JSON responses.

---

## High (Data Integrity)

### ~~6. No transactions on registration~~ ✅ DONE
The register flow does 4+ DB writes (user, active_code, access, auth_code) without a transaction. A failure mid-way leaves orphaned records.
**Fix:** Wrapped the entire register flow in a SeaORM transaction using `_with_txn` repo methods.
**See:** `features/auth/service/src/authentication.rs`, `.kiro/docs/dev/transaction-pattern.md`

### ~~7. Kafka sent before DB completion~~ ✅ DONE
If auth_code creation fails after the Kafka SignUp event is sent, the notification pipeline fires for a registration that didn't complete.
**Fix:** Moved Kafka publish to after `txn.commit()`. Kafka failure is now non-fatal (logged, doesn't fail registration).

---

## Medium (Correctness/Maintenance)

### 8. No integration tests
The auth API has zero tests. Add tests for:
- Login flow (happy path + wrong password)
- Token exchange + refresh + revocation
- Permission guard rejection
- Registration with transaction rollback scenarios

### 9. Hardcoded pagination limit (200) for permission-sync
If a role has >200 permissions, the sync silently truncates.
**Fix:** Paginate fully or remove the cap.

### 10. No graceful shutdown for permission-polling task
The spawned 30s polling loop runs indefinitely.
**Fix:** Use a `CancellationToken` so it stops on server shutdown.

---

## High (Security)

### 13. Token invalidation on client_secret update
When `client_secret` is updated via `PATCH /clients/{id}`, all existing tokens for that client become unverifiable (signed with old secret) but cached JTIs remain valid, causing inconsistency.
**Fix:** When client_secret changes:
- Clear all token cache entries (access + refresh JTIs) for users of that client
- Revoke all tokens in the DB for that client (`tokens.revoked_at = now()`)
- Optionally require the old secret or a confirmation flag in the update request to prevent accidental rotation

### 14. Require old secret verification on client_secret update
To prevent accidental secret rotation without understanding the impact, require the current `client_secret` (or a confirmation flag) in the PATCH request body before allowing the change.
**Fix:** Add an `old_client_secret` field to `ClientForUpdateRequest`; validate it matches the stored value before applying the update.

### 15. Client secret stored in plaintext (acceptable)
`client_secret` is stored in plaintext because it's used as a JWT HMAC signing key — it must be retrievable. This is the standard OAuth2 client credentials pattern (same as AWS/GCP client secrets). No fix needed, but document this design decision.

### 16. Client secret minimum entropy
The validator enforces 10–128 chars length, but doesn't enforce complexity (e.g., mixed case, digits, special chars).
**Fix:** Consider generating the secret server-side (random 32+ byte base64) instead of accepting user-provided secrets, or add a complexity validator.

---

## Low (Code Quality)

### 11. Add `#[instrument]` tracing
Add tracing spans to service-layer functions for observability.

### 12. Extract common CRUD patterns
Permissions, scopes, clients all follow identical create/read/update/delete patterns. Consider a macro or generic handler to reduce boilerplate.

---

## Recommended Fix Order

1. ~~Items 1–3 (quick fixes, prevent crashes and security holes)~~ ✅ DONE
2. ~~Items 6–7 (data integrity)~~ ✅ DONE
3. Items 13–14 (token invalidation and secret rotation safety)
4. Items 4 and 8 (rate limiting and tests)
5. Items 9–10 (correctness/maintenance)
6. Items 11–12, 15–16 (code quality and documentation)
