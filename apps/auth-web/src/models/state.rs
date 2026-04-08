#[derive(Debug)]
#[cfg(feature = "server")]
pub struct AppState {
    title: String,
}

#[cfg(feature = "server")]
impl AppState {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}
