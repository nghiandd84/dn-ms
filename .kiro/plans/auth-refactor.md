# Auth API Refactoring Plan

## 🔴 High Priority

### 1. Fix `.unwrap()` in `login.rs` (Bug — Server Crash)
- **File:** `apis/auth/src/routes/login.rs`
- **Problem:** `LoginService::login(login_request).await.unwrap()` panics on login failure
- **Fix:** Replace with `?` operator for proper error propagation

## 🟡 Medium Priority

### 2. Simplify error handling in `register.rs`
- **File:** `apis/auth/src/routes/register.rs`
- **Problem:** Manual if/let + `.err().unwrap()` pattern is verbose and inconsistent
- **Fix:** Use `.map_err(AppError::Auth)?` for idiomatic error propagation

### 3. Extract permission-sync from `custom_handler`
- **File:** `apis/auth/src/app.rs`
- **Problem:** `custom_handler` mixes Kafka producer setup with a background permission-polling loop
- **Fix:** Extract the permission-sync loop into a dedicated function/module for testability

### 4. CRUD boilerplate reduction (optional)
- **Files:** `client.rs`, `scope.rs`, `permission.rs`, `auth_code.rs`
- **Problem:** ~80 lines of near-identical CRUD code per resource
- **Fix:** Consider a macro or generic handler if adding more CRUD resources

## 🟢 Low Priority

### 5. Fix typo in debug log
- **File:** `apis/auth/src/routes/login.rs`
- **Problem:** `"Login requet"` → should be `"Login request"`

### 6. Consistent `#[instrument]` usage
- **Files:** All route handlers
- **Problem:** Only `filter_users` has `#[instrument]`; others lack tracing instrumentation
- **Fix:** Add `#[instrument]` to all handlers or remove from `filter_users` for consistency

### 7. Add unit tests
- **Problem:** No tests exist for auth API handlers
- **Fix:** Add test modules covering login, register, token creation/verification, and permission assignment
