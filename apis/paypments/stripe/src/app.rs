use axum::Router;
use std::sync::Arc;
use stripe::Client;
use tokio::net::TcpListener;
use tracing::info;

use crate::routes;

pub struct AppState {
    pub stripe_client: Client,
}

pub async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let stripe_secret = std::env::var("STRIPE_SECRET_KEY")?;
    let client = Client::new(stripe_secret);

    let state = Arc::new(AppState {
        stripe_client: client,
    });

    let app = routes::routes().with_state(state);

    let port = std::env::var("PORT").unwrap_or("3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Stripe API running on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}