use app::start_app;

mod app;
mod doc;
mod routes;
mod consumers;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
}
