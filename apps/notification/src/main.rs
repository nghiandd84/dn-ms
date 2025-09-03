use crate::app::start_app;

mod app;
mod email;
mod consumer;
mod websocket;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
    // tokio::spawn(async move {
    //     consumer::cusumer_task().await.unwrap();
    // });
}
