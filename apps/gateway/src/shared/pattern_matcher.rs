use std::fmt::Debug;

use pcre2::bytes::Regex;

use crate::error::{BErrorStd, DakiaResult};

#[derive(Debug, Clone)]
pub struct Pcre2PatternMatcher {
    regex: Regex,
}

impl Pcre2PatternMatcher {
    pub fn build(pattern: &str) -> DakiaResult<Self> {
        let pcre2regex = Regex::new(pattern)?;
        let matcher = Self { regex: pcre2regex };
        Ok(matcher)
    }
}

impl PatternMatcher for Pcre2PatternMatcher {
    fn is_match(&self, text: &[u8]) -> Result<bool, BErrorStd> {
        let is_matched = self.regex.is_match(text)?;
        Ok(is_matched)
    }
}

pub trait PatternMatcher: Send + Sync + Debug {
    fn is_match(&self, text: &[u8]) -> Result<bool, BErrorStd>;
}
