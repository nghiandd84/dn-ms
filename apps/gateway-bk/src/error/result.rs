use std::sync::PoisonError;
use tokio::sync::RwLockReadGuard;

use super::{DakiaError, ImmutStr};

pub type DakiaResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub enum Error {
    DakiaError(DakiaError),
    PoisonError(String),
    PingoraError(pingora_core::Error),
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for Error {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

impl From<pingora_core::Error> for Error {
    fn from(err: pingora_core::Error) -> Self {
        Error::PingoraError(err)
    }
}

impl From<Box<pingora::Error>> for Box<Error> {
    fn from(err: Box<pingora_core::Error>) -> Self {
        // TODO: format this error
        err.into()
    }
}

impl From<http::status::InvalidStatusCode> for Box<Error> {
    fn from(err: http::status::InvalidStatusCode) -> Self {
        // TODO: format this error
        err.into()
    }
}

impl From<DakiaError> for Error {
    fn from(err: DakiaError) -> Self {
        Error::DakiaError(err)
    }
}

impl From<Box<Error>> for Box<pingora_core::Error> {
    fn from(value: Box<Error>) -> Box<pingora_core::Error> {
        match *value {
            Error::PingoraError(pe) => Box::new(pe),
            Error::DakiaError(de) => {
                // TODO: handle translation between pingora and dakia error
                // pass status code correctly
                // currently, it'll just print error message. Which is enough for debugging for now...
                let error_msg = de.to_string();
                let pe = pingora_core::Error::explain(pingora::ErrorType::InternalError, error_msg);
                pe
            }
            // TODO: implement conversion for other errors
            _ => pingora_core::Error::new(pingora::ErrorType::InternalError),
        }
    }
}

impl From<std::io::Error> for Box<Error> {
    fn from(value: std::io::Error) -> Self {
        let message = value.to_string().into_boxed_str();
        DakiaError::create_unknown_context(ImmutStr::Owned(message))
    }
}

impl From<pcre2::Error> for Box<Error> {
    fn from(value: pcre2::Error) -> Self {
        let message = value.to_string().into_boxed_str();
        DakiaError::create_unknown_context(ImmutStr::Owned(message))
    }
}

impl From<http::Error> for Box<Error> {
    fn from(value: http::Error) -> Self {
        let message = value.to_string().into_boxed_str();
        DakiaError::create_unknown_context(ImmutStr::Owned(message))
    }
}
