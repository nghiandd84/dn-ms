use std::fmt;

#[derive(Debug, Clone)]
pub enum ConsumerError {
    NotFound { message: String },
    SendEmailError { message: String },
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsumerError::NotFound { message } => {
                write!(f, "Not found: {}", message)
            }
            ConsumerError::SendEmailError { message } => {
                write!(f, "Failed to send email: {}", message)
            }
        }
    }
}

impl std::error::Error for ConsumerError {}
