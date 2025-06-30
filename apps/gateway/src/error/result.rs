pub type GatewayResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    PoisonError { message: &'static str },
}

impl Error {
    pub fn from_str(message: &'static str) -> Self {
        Error::PoisonError { message }
    }
}
