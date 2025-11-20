#[derive(Debug)]
pub struct AppState {
    title: String,
}

impl AppState {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}
