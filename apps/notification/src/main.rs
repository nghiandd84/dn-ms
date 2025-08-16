use crate::app::start_app;

mod app;
mod email;
mod websocket;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
}
