#![warn(clippy::all)]
//! The library to provide the struct to represent errors in pingora.

pub use std::error::Error as ErrorTrait;
use std::fmt;
use std::fmt::Debug;
use std::result::Result as StdResult;

mod immut_str;
mod result;
pub use immut_str::ImmutStr;
pub use result::*;

/// The boxed [Error], the desired way to pass [Error]
pub type BError = Box<Error>;
pub type BErrorStd = Box<dyn std::error::Error>;

/// Syntax sugar for `std::Result<T, BError>`
pub type Result<T, E = BError> = StdResult<T, E>;

/// The struct that represents an error
#[derive(Debug)]
pub struct DakiaError {
    /// the type of error
    pub etype: ErrorType,
    /// the source of error: from upstream, downstream or internal
    pub source: ErrorSource,
    /// chain to the cause of this error
    pub cause: Option<Box<(dyn ErrorTrait + Send + Sync)>>,
    /// an arbitrary string that explains the context when the error happens
    pub context: Option<ImmutStr>,
}

/// The source of the error
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorSource {
    /// The error is caused by the remote server
    Upstream,
    /// The error is caused by the remote client
    Downstream,
    /// The error is caused by the internal logic
    Internal,
    /// Error source unknown or to be set
    Unknown,
}

impl ErrorSource {
    /// for displaying the error source
    pub fn as_str(&self) -> &str {
        match self {
            Self::Upstream => "Upstream",
            Self::Downstream => "Downstream",
            Self::Internal => "Internal",
            Self::Unknown => "Unknown",
        }
    }
}

/// Predefined type of errors
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorType {
    // error occurred during proxy
    ProxyError(u16),
    // other errors
    InternalError,
    // catch all
    UnknownError,
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorType::ProxyError(_) => "HTTPStatus",
            ErrorType::InternalError => "InternalError",
            ErrorType::UnknownError => "UnknownError",
        }
    }
}

impl DakiaError {
    /// Simply create the error. See other functions that provide less verbose interfaces.
    #[inline]
    pub fn create(
        etype: ErrorType,
        esource: ErrorSource,
        context: Option<ImmutStr>,
        cause: Option<Box<dyn ErrorTrait + Send + Sync>>,
    ) -> BError {
        let de = DakiaError {
            etype: etype,
            source: esource,
            cause,
            context,
        };

        Box::new(Error::DakiaError(de))
    }

    /// Simply create the error. See other functions that provide less verbose interfaces.
    #[inline]
    pub fn create_internal() -> BError {
        let de = DakiaError {
            etype: ErrorType::InternalError,
            source: ErrorSource::Internal,
            cause: None,
            context: None,
        };

        Box::new(Error::DakiaError(de))
    }

    /// Simply create the error. See other functions that provide less verbose interfaces.
    #[inline]
    pub fn create_internal_context(context: &'static str) -> BError {
        let de = DakiaError {
            etype: ErrorType::InternalError,
            source: ErrorSource::Internal,
            cause: None,
            context: Some(ImmutStr::Static(context)),
        };

        Box::new(Error::DakiaError(de))
    }

    /// Simply create the error. See other functions that provide less verbose interfaces.
    #[inline]
    pub fn create_unknown_context(context: ImmutStr) -> BError {
        let de = DakiaError {
            etype: ErrorType::UnknownError,
            source: ErrorSource::Unknown,
            cause: None,
            context: Some(context),
        };

        Box::new(Error::DakiaError(de))
    }

    /// Simply create the error. See other functions that provide less verbose interfaces.
    #[inline]
    pub fn create_unknown_msg(msg: &str) -> BError {
        let context = ImmutStr::Owned(msg.to_string().into_boxed_str());

        let de = DakiaError {
            etype: ErrorType::UnknownError,
            source: ErrorSource::Unknown,
            cause: None,
            context: Some(context),
        };

        Box::new(Error::DakiaError(de))
    }

    #[inline]
    fn do_new(e: ErrorType, s: ErrorSource) -> BError {
        Self::create(e, s, None, None)
    }

    /// Create an error with the given type
    #[inline]
    pub fn new(e: ErrorType) -> BError {
        Self::do_new(e, ErrorSource::Unknown)
    }

    #[inline]
    pub fn because<S: Into<ImmutStr>, E: Into<Box<dyn ErrorTrait + Send + Sync>>>(
        e: ErrorType,
        context: S,
        cause: E,
    ) -> BError {
        Self::create(
            e,
            ErrorSource::Unknown,
            Some(context.into()),
            Some(cause.into()),
        )
    }

    // Short for Err(Self::because)
    #[inline]
    pub fn e_because<T, S: Into<ImmutStr>, E: Into<Box<dyn ErrorTrait + Send + Sync>>>(
        e: ErrorType,
        context: S,
        cause: E,
    ) -> Result<T> {
        Err(Self::because(e, context, cause))
    }

    // Create an error with context but no direct causing error
    #[inline]
    pub fn explain<S: Into<ImmutStr>>(e: ErrorType, context: S) -> BError {
        Self::create(e, ErrorSource::Unknown, Some(context.into()), None)
    }

    // Create an internal error with context but no direct causing error
    #[inline]
    pub fn i_explain<S: Into<ImmutStr>>(context: S) -> BError {
        Self::create(
            ErrorType::InternalError,
            ErrorSource::Unknown,
            Some(context.into()),
            None,
        )
    }

    // Short for Err(Self::explain)
    #[inline]
    pub fn e_explain<T, S: Into<ImmutStr>>(e: ErrorType, context: S) -> Result<T> {
        Err(Self::explain(e, context))
    }

    // The new_{up, down, in} functions are to create new errors with source
    // {upstream, downstream, internal}
    #[inline]
    pub fn new_up(e: ErrorType) -> BError {
        Self::do_new(e, ErrorSource::Upstream)
    }

    #[inline]
    pub fn new_down(e: ErrorType) -> BError {
        Self::do_new(e, ErrorSource::Downstream)
    }

    #[inline]
    pub fn new_in(e: ErrorType) -> BError {
        Self::do_new(e, ErrorSource::Internal)
    }

    // the err_* functions are the same as new_* but return a Result<T>
    #[inline]
    pub fn err<T>(e: ErrorType) -> Result<T> {
        Err(Self::new(e))
    }

    #[inline]
    pub fn err_up<T>(e: ErrorType) -> Result<T> {
        Err(Self::new_up(e))
    }

    #[inline]
    pub fn err_down<T>(e: ErrorType) -> Result<T> {
        Err(Self::new_down(e))
    }

    #[inline]
    pub fn err_in<T>(e: ErrorType) -> Result<T> {
        Err(Self::new_in(e))
    }

    pub fn etype(&self) -> &ErrorType {
        &self.etype
    }

    pub fn esource(&self) -> &ErrorSource {
        &self.source
    }

    pub fn reason_str(&self) -> &str {
        self.etype.as_str()
    }

    pub fn source_str(&self) -> &str {
        self.source.as_str()
    }

    /// The as_{up, down, in} functions are to change the current errors with source
    /// {upstream, downstream, internal}
    pub fn as_up(&mut self) {
        self.source = ErrorSource::Upstream;
    }

    pub fn as_down(&mut self) {
        self.source = ErrorSource::Downstream;
    }

    pub fn as_in(&mut self) {
        self.source = ErrorSource::Internal;
    }

    pub fn set_cause<C: Into<Box<dyn ErrorTrait + Send + Sync>>>(&mut self, cause: C) {
        self.cause = Some(cause.into());
    }

    pub fn set_context<T: Into<ImmutStr>>(&mut self, context: T) {
        self.context = Some(context.into());
    }

    // Display error but skip the duplicate elements from the error in previous hop
    fn chain_display(
        &self,
        previous: Option<&DakiaError>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if previous.map(|p| p.source != self.source).unwrap_or(true) {
            write!(f, "{}", self.source.as_str())?
        }
        if previous.map(|p| p.etype != self.etype).unwrap_or(true) {
            write!(f, " {}", self.etype.as_str())?
        }

        if let Some(c) = self.context.as_ref() {
            write!(f, " context: {}", c)?;
        }
        if let Some(c) = self.cause.as_ref() {
            if let Some(e) = c.downcast_ref::<Box<DakiaError>>() {
                write!(f, " cause: ")?;
                e.chain_display(Some(self), f)
            } else {
                write!(f, " cause: {}", c)
            }
        } else {
            Ok(())
        }
    }

    pub fn to_pingora_error(self) -> Box<pingora::Error> {
        let petype = match self.etype {
            ErrorType::InternalError => pingora::ErrorType::InternalError,
            ErrorType::ProxyError(status_code) => pingora::ErrorType::HTTPStatus(status_code),
            ErrorType::UnknownError => pingora::ErrorType::UnknownError,
        };

        let pesource = match self.source {
            ErrorSource::Downstream => pingora::ErrorSource::Downstream,
            ErrorSource::Internal => pingora::ErrorSource::Internal,
            ErrorSource::Unknown => pingora::ErrorSource::Unset,
            ErrorSource::Upstream => pingora::ErrorSource::Upstream,
        };

        let pe = pingora::Error::create(
            petype, pesource, None,
            // TODO: handle conversion of context
            // Some(pingora::ImmutStr::Owned(
            //     self.context.unwrap().clone().to_string().into_boxed_str(),
            // )),
            self.cause,
        );
        pe
    }
}

impl fmt::Display for DakiaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.chain_display(None, f)
    }
}

impl ErrorTrait for DakiaError {}

// Helper trait to add more context to a given error
pub trait Context<T> {
    // Wrap the `Err(E)` in [Result] with more context, the existing E will be the cause.
    // This is a shortcut for map_err() + more_context()
    fn err_context<C: Into<ImmutStr>, F: FnOnce() -> C>(self, context: F) -> Result<T, BError>;
}

// Helper trait to chain errors with context
pub trait OrErr<T, E> {
    // Wrap the E in [Result] with new [ErrorType] and context, the existing E will be the cause.
    // This is a shortcut for map_err() + because()
    fn or_err(self, et: ErrorType, context: &'static str) -> Result<T, BError>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>;

    // Similar to or_err(), but takes a closure, which is useful for constructing String.
    fn or_err_with<C: Into<ImmutStr>, F: FnOnce() -> C>(
        self,
        et: ErrorType,
        context: F,
    ) -> Result<T, BError>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>;

    // Replace the E in [Result] with a new [Error] generated from the current error
    // This is useful when the current error cannot move out of scope. This is a shortcut for map_err() + explain().
    fn explain_err<C: Into<ImmutStr>, F: FnOnce(E) -> C>(
        self,
        et: ErrorType,
        context: F,
    ) -> Result<T, BError>;

    // Similar to or_err() but just to surface errors that are not [Error] (where `?` cannot be used directly).
    // or_err()/or_err_with() are still preferred because they make the error more readable and traceable.
    fn or_fail(self) -> Result<T>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>;
}

impl<T, E> OrErr<T, E> for Result<T, E> {
    fn or_err(self, et: ErrorType, context: &'static str) -> Result<T, BError>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>,
    {
        self.map_err(|e| DakiaError::because(et, context, e))
    }

    fn or_err_with<C: Into<ImmutStr>, F: FnOnce() -> C>(
        self,
        et: ErrorType,
        context: F,
    ) -> Result<T, BError>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>,
    {
        self.map_err(|e| DakiaError::because(et, context(), e))
    }

    fn explain_err<C: Into<ImmutStr>, F: FnOnce(E) -> C>(
        self,
        et: ErrorType,
        exp: F,
    ) -> Result<T, BError> {
        self.map_err(|e| DakiaError::explain(et, exp(e)))
    }

    fn or_fail(self) -> Result<T, BError>
    where
        E: Into<Box<dyn ErrorTrait + Send + Sync>>,
    {
        self.map_err(|e| DakiaError::because(ErrorType::InternalError, "", e))
    }
}

// Helper trait to convert an [Option] to an [Error] with context.
pub trait OkOrErr<T> {
    fn or_err(self, et: ErrorType, context: &'static str) -> Result<T, BError>;

    fn or_err_with<C: Into<ImmutStr>, F: FnOnce() -> C>(
        self,
        et: ErrorType,
        context: F,
    ) -> Result<T, BError>;
}

impl<T> OkOrErr<T> for Option<T> {
    // Convert the [Option] to a new [Error] with [ErrorType] and context if None, Ok otherwise.
    // This is a shortcut for .ok_or(Error::explain())
    fn or_err(self, et: ErrorType, context: &'static str) -> Result<T, BError> {
        self.ok_or(DakiaError::explain(et, context))
    }

    // Similar to to_err(), but takes a closure, which is useful for constructing String.
    fn or_err_with<C: Into<ImmutStr>, F: FnOnce() -> C>(
        self,
        et: ErrorType,
        context: F,
    ) -> Result<T, BError> {
        self.ok_or_else(|| DakiaError::explain(et, context()))
    }
}
