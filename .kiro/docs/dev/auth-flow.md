# Auth Flow Documentation

## Overview

The auth system uses a 2-step verification approach for both registration and login. Both flows send a 6-digit code via Kafka (for email delivery) and require code verification before returning an auth_code.

## OpenAPI Security

The API uses a `baggage` header instead of Bearer token for Swagger UI authentication:
```
baggage: accesses=ADMIN_ALL*,user_id=00000000-0000-0000-0000-000000000000,client_id=00000000-0000-0000-0000-000000000000
```

Defined in `libs/shared/shared/app/src/doc.rs` as `JwtSecurityAddon` using `SecurityScheme::ApiKey(ApiKey::Header(...))`.

---

## Registration Flow

### Step 1: Register — `POST /public/requests/register`

**Request:** `AuthRegisterRequest { email, password, state, language }`

**Process:**
1. Validate state (lookup authentication_request by state UUID)
2. Create user (is_active = false)
3. Generate 6-digit activation code, store in `active_codes` table (10 min expiry)
4. Assign default role to user
5. Create auth_code from authentication_request data (client_id, redirect_uri, scopes)
6. Send Kafka `SignUp::Success` event (contains active_code for email delivery)

**Response:** `{ user_id, id_token (auth_code), redirect_uri }`

### Step 2: Activate — `POST /public/signup/active`

**Request:** `SignupActiveRequest { user_id, code }`

**Process:**
1. Find active_code by user_id + code + is_used=false
2. Verify not expired (10 min window)
3. Mark active_code as used
4. Set user.is_active = true
5. Look up auth_code by user_id

**Response:** `{ ok, auth_code, redirect_uri }`

---

## Login Flow

### Step 1: Login — `POST /public/requests/login`

**Request:** `AuthLoginRequest { email, password, state }`

**Process:**
1. Validate state (lookup authentication_request by state UUID)
2. Validate credentials (email + password)
3. Create auth_code from authentication_request data (client_id, redirect_uri, scopes)
4. Generate 6-digit login code, store in `active_codes` table (10 min expiry)
5. Send Kafka `SignIn::LoginCode` event (contains login_code, user_id, email)

**Response:** `{ user_id }`

### Step 2: Verify Login Code — `POST /public/login/code`

**Request:** `LoginCodeRequest { user_id, login_code }`

**Process:**
1. Find active_code by user_id + code + is_used=false
2. Verify not expired (10 min window)
3. Mark active_code as used
4. Look up auth_code by user_id (created during step 1, contains redirect_uri from authentication_request)

**Response:** `{ auth_code, redirect_uri }`

---

## Kafka Events

### SignUp::Success
```json
{
  "auth_type": "sign_up",
  "message": {
    "signup_type": "success",
    "user_id": "uuid",
    "email": "user@example.com",
    "app_key": "client_key",
    "active_code": "123456",
    "language_code": "en-US",
    "client_email": "admin@app.com"
  }
}
```

### SignIn::LoginCode
```json
{
  "auth_type": "sign_in",
  "message": {
    "signin_type": "login_code",
    "user_id": "uuid",
    "email": "user@example.com",
    "login_code": "654321"
  }
}
```

---

## Key Implementation Files

| Layer | File |
|-------|------|
| Route handlers | `apis/auth/src/routes/authentication.rs`, `apis/auth/src/routes/signup.rs` |
| Models | `features/auth/model/src/authentication.rs`, `features/auth/model/src/login.rs`, `features/auth/model/src/signup.rs` |
| Services | `features/auth/service/src/authentication.rs`, `features/auth/service/src/login.rs`, `features/auth/service/src/active_code.rs` |
| Stream/Kafka | `features/auth/stream/src/signin.rs`, `features/auth/stream/src/signup.rs` |
| Entities | `features/auth/entities/src/active_code.rs`, `features/auth/entities/src/user.rs` |

## active_codes Table

Used for both registration activation codes and login verification codes.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK to users |
| code | String(250) | unique, 6-digit numeric |
| is_used | bool | default false |
| is_sent | bool | default false, set true after Kafka delivery confirmed |
| expiration_time | DateTime | created_at + 10 minutes |
| created_at | DateTime | auto |
| updated_at | DateTime | auto |
