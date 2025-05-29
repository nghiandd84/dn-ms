pub mod result;
use core::result::Result as CoreResult;

pub type Result<T> = CoreResult<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    PoisonError { message: String },
}

impl Error {
    pub fn from_str(str: String) -> Self {
        Error::PoisonError { message: str }
    }
}
