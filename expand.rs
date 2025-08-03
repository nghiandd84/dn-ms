use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet; // Using HashSet to simulate a simple cache for user IDs
use std::sync::Arc; // For sharing AppState across threads
use tokio::sync::Mutex; // For mutable access to the cache in a concurrent environment

// Define a custom error type for our extractor
#[derive(Debug)]
pub enum UserIdError {
    MissingHeader,
    EmptyHeader,
    Unauthorized, // Added for cache check failure
    InternalServerError, // For unexpected errors during cache lookup
}

// Implement IntoResponse for our custom error, so it can be returned directly from the extractor.
// This will map our custom errors to HTTP responses.
impl IntoResponse for UserIdError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserIdError::MissingHeader => (StatusCode::UNAUTHORIZED, "Missing 'dn_user_id' header"),
            UserIdError::EmptyHeader => (StatusCode::UNAUTHORIZED, "Empty 'dn_user_id' header"),
            UserIdError::Unauthorized => (StatusCode::UNAUTHORIZED, "User ID not authorized or found in cache"),
            UserIdError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error during user ID validation"),
        };

        // Return a JSON error response
        let body = Json(serde_json::json!({
            "error": error_message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Define the struct that will hold our extracted user ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

// Define your application state. This struct would hold your cache client, database pool, etc.
// For this example, we'll use a simple in-memory HashSet to simulate a cache of authorized user IDs.
#[derive(Clone)]
pub struct AppState {
    // In a real application, this would be your Redis client, database connection pool, etc.
    // We use Arc<Mutex<...>> to allow mutable access to the HashSet across async tasks.
    pub authorized_users_cache: Arc<Mutex<HashSet<String>>>,
}

// Implement the FromRequestParts trait for our UserId struct, now generic over the state `S`.
#[async_trait]
impl<S> FromRequestParts for UserId
where
    S: Send + Sync + 'static, // S must be Send + Sync + 'static for use with Axum State
    AppState: From<S>, // This trait bound ensures that AppState can be created from S,
                       // or more simply, if S *is* AppState, this is satisfied.
{
    type Rejection = UserIdError;

    async fn from_request_parts(parts: &mut Parts, state: &State<S>) -> Result<Self, Self::Rejection> {
        // Try to get the 'dn_user_id' header from the request parts.
        let header_value = parts.headers.get("dn_user_id");

        let user_id_str = match header_value {
            Some(value) => {
                let s = value.to_str().map_err(|_| UserIdError::MissingHeader)?;
                if s.is_empty() {
                    return Err(UserIdError::EmptyHeader);
                }
                s.to_string()
            }
            None => {
                return Err(UserIdError::MissingHeader);
            }
        };

        // --- Cache Check Logic (using the generic state) ---
        // Assuming `state.0` gives us access to the AppState.
        // In a real scenario, `S` would likely be `AppState` directly.
        let app_state: &AppState = state.0.as_ref(); // Cast S to AppState if S is a wrapper or reference

        let cache = app_state.authorized_users_cache.lock().await;
        if !cache.contains(&user_id_str) {
            // If the user ID is not in our simulated cache, reject the request.
            return Err(UserIdError::Unauthorized);
        }
        // --- End Cache Check Logic ---

        Ok(UserId(user_id_str))
    }
}

// A simple handler that uses our custom UserId extractor.
async fn protected_route(user_id: UserId) -> (StatusCode, String) {
    // If we reach here, the user_id has been successfully extracted, is not empty,
    // and was found in the simulated cache.
    (
        StatusCode::OK,
        format!("Access granted for user: {}", user_id.0),
    )
}

// A handler that demonstrates what happens if the header is missing/empty.
async fn public_route() -> (StatusCode, String) {
    (StatusCode::OK, "This is a public route.".to_string())
}

// Main function to set up and run the Axum server.
#[tokio::main]
async fn main() {
    // Initialize our application state with some authorized users
    let mut initial_authorized_users = HashSet::new();
    initial_authorized_users.insert("user123".to_string());
    initial_authorized_users.insert("admin".to_string());

    let app_state = AppState {
        authorized_users_cache: Arc::new(Mutex::new(initial_authorized_users)),
    };

    // Build our application with routes that use the extractor and pass the state.
    let app = axum::Router::new()
        .route("/protected", axum::routing::get(protected_route))
        .route("/public", axum::routing::get(public_route))
        .with_state(app_state); // Attach the state to the router

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

/*
To test this:

1.  **Save the code:** Save this as `src/main.rs` in a new Rust project (e.g., `cargo new my-axum-app --bin`).
2.  **Add dependencies to `Cargo.toml`:**
    ```toml
    [package]
    name = "my-axum-app"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    axum = "0.7"
    tokio = { version = "1", features = ["full"] }
    http = "1.0" # axum's http dependency
    async-trait = "0.1"
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    # For the simulated cache
    std = { version = "1.70.0", features = ["collections"] } # Ensure you have collections feature enabled for HashSet
    ```
    Note: `std` is usually implicitly available. You might only need to explicitly add `tokio` with `full` features if you encounter issues with `Mutex`. `std::collections` is part of the standard library, so you typically don't need to add it to `Cargo.toml`. The `tokio` dependency with `full` features should be sufficient.

3.  **Run the application:** `cargo run`

**Example cURL requests:**

* **Success (valid header and authorized in cache):**
    ```bash
    curl -v -H "dn_user_id: user123" http://localhost:3000/protected
    ```
    Expected output: `HTTP/1.1 200 OK` and `Access granted for user: user123`

* **Error (missing header):**
    ```bash
    curl -v http://localhost:3000/protected
    ```
    Expected output: `HTTP/1.1 401 Unauthorized` and `{"code":401,"error":"Missing 'dn_user_id' header"}`

* **Error (empty header):**
    ```bash
    curl -v -H "dn_user_id:" http://localhost:3000/protected
    ```
    Expected output: `HTTP/1.1 401 Unauthorized` and `{"code":401,"error":"Empty 'dn_user_id' header"}`

* **Error (header present but NOT authorized in cache):**
    ```bash
    curl -v -H "dn_user_id: unauthorized_user" http://localhost:3000/protected
    ```
    Expected output: `HTTP/1.1 401 Unauthorized` and `{"code":401,"error":"User ID not authorized or found in cache"}`

* **Public route (no header needed):**
    ```bash
    curl -v http://localhost:3000/public
    ```
    Expected output: `HTTP/1.1 200 OK` and `This is a public route.`
*/
