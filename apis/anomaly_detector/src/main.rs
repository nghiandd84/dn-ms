use std::sync::Arc;

use axum::Router;
use redis::Client;
use tokio::net::TcpListener;
use tracing::info;

mod handlers;
mod models;
mod routes;

pub struct AppState {
    pub redis: Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("ANOMALY_DETECTOR_REDIS_URL")
        .or_else(|_| std::env::var("REDIS_URL"))
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis = Client::open(redis_url).expect("Failed to connect to Redis");

    let state = Arc::new(AppState { redis });

    let app = Router::new()
        .merge(routes::routes(state))
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let port = std::env::var("ANOMALY_DETECTOR_PORT").unwrap_or_else(|_| "5401".to_string());
    let addr = format!("0.0.0.0:{}", port);
    info!("Anomaly Detector API listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
