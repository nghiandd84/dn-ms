use std::fmt;

// A data struct that holds either immutable string or reference to static str.
// Compared to String or `Box<str>`, it avoids memory allocation on static str.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImmutStr {
    Static(&'static str),
    Owned(Box<str>),
}

impl ImmutStr {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            ImmutStr::Static(s) => s,
            ImmutStr::Owned(s) => s.as_ref(),
        }
    }

    pub fn is_owned(&self) -> bool {
        match self {
            ImmutStr::Static(_) => false,
            ImmutStr::Owned(_) => true,
        }
    }
}

impl fmt::Display for ImmutStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&'static str> for ImmutStr {
    fn from(s: &'static str) -> Self {
        ImmutStr::Static(s)
    }
}

impl From<String> for ImmutStr {
    fn from(s: String) -> Self {
        ImmutStr::Owned(s.into_boxed_str())
    }
}
