use app::start_app;

mod app;
mod doc;
mod error_page;
mod middleware;
mod permission;
mod routes;

#[tokio::main]
async fn main() {
    start_app().await.unwrap();
}
