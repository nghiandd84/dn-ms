pub type GatewayResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    PoisonError { message: String },
}

impl Error {
    pub fn from_str(str: String) -> Self {
        Error::PoisonError { message: str }
    }
}
