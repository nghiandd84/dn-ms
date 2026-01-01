use crate::app::start_app;

mod app;
mod consumer;
mod email;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
}
