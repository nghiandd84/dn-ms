use std::fmt::Display;

#[derive(Debug)]
pub enum TokenError {
    InvalidToken,
    ExpiredToken,
    UnauthorizedAccess,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::InvalidToken => write!(f, "The provided token is invalid."),
            TokenError::ExpiredToken => write!(f, "The token has expired."),
            TokenError::UnauthorizedAccess => write!(f, "Unauthorized access attempt."),
        }
    }
}
