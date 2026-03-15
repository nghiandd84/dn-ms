// use tracing_subscriber;

use app::start_app;

mod app;
mod routes;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    start_app().await.unwrap();
}