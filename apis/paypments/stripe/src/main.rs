use app::start_app;

mod app;
mod consumers;
mod doc;
mod routes;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
}
