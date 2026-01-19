use std::fmt::Formatter;

pub type GatewayResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PoisonError { message: &'static str },
}

impl Error {
    pub fn from_str(message: &'static str) -> Self {
        Error::PoisonError { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoisonError { message } => write!(f, "PoisonError: {}", message),
        }
    }
}
