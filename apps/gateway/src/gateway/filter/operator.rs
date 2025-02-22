use crate::{error::Error, qe::query::Query, shared::pattern_matcher::Pcre2PatternMatcher};

use super::query2filter::query2filter;

#[derive(Debug, Clone)]
pub enum RelationalOperator {
    Eq(Vec<u8>),
    Ne(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum SetOperator {
    In(Vec<Vec<u8>>),
    Nin(Vec<Vec<u8>>),
}

#[derive(Debug, Clone)]
pub enum PatternOperator {
    Contains(Vec<u8>),
    NotContains(Vec<u8>),
    StartsWith(Vec<u8>),
    NotStartWith(Vec<u8>),
    EndsWith(Vec<u8>),
    NotEndsWith(Vec<u8>),
    Matches(Pcre2PatternMatcher),
}

#[derive(Debug, Clone)]
pub enum Header {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    CacheControl,
    ContentType,
    ContentLength,
    SetCookie,
    Host,
    Origin,
    Referer,
    UserAgent,
    XForwardedFor,
    XRequestId,
    Custom(Vec<u8>), // Allows custom headers
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "accept" => Header::Accept,
            "accept-encoding" => Header::AcceptEncoding,
            "accept-language" => Header::AcceptLanguage,
            "authorization" => Header::Authorization,
            "cache-control" => Header::CacheControl,
            "content-type" => Header::ContentType,
            "content-length" => Header::ContentLength,
            "set-cookie" => Header::SetCookie,
            "host" => Header::Host,
            "origin" => Header::Origin,
            "referer" => Header::Referer,
            "user-agent" => Header::UserAgent,
            "x-forwarded-for" => Header::XForwardedFor,
            "x-request-id" => Header::XRequestId,
            _ => Header::Custom(value.as_bytes().to_vec()),
        }
    }
}

impl Header {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Header::Accept => b"accept",
            Header::AcceptEncoding => b"accept-encoding",
            Header::AcceptLanguage => b"accept-language",
            Header::Authorization => b"authorization",
            Header::CacheControl => b"cache-control",
            Header::ContentType => b"content-type",
            Header::ContentLength => b"content-length",
            Header::SetCookie => b"set-cookie",
            Header::Host => b"host",
            Header::Origin => b"origin",
            Header::Referer => b"referer",
            Header::UserAgent => b"user-agent",
            Header::XForwardedFor => b"x-forwarded-for",
            Header::XRequestId => b"x-request-id",
            Header::Custom(bytes) => bytes,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CriteriaOperator {
    Relation(RelationalOperator),
    Pattern(PatternOperator),
    Set(SetOperator),
    Exists(bool),
}

#[derive(Debug, Clone)]
pub enum LogicalCriteriaOperator {
    And(Vec<CriteriaOperator>),
    Or(Vec<CriteriaOperator>),
}

#[derive(Debug, Clone)]
pub enum PartCriteriaOperator {
    CriteriaOperator(CriteriaOperator),
    LogicalCriteriaOperator(LogicalCriteriaOperator),
}

#[derive(Debug, Clone)]
pub struct HeaderCriteria {
    pub name: Header,
    pub operator: Vec<PartCriteriaOperator>,
}

#[derive(Debug, Clone)]
pub struct QueryCriteria {
    pub name: Vec<u8>,
    pub operator: Vec<PartCriteriaOperator>,
}

#[derive(Debug, Clone)]
pub enum PartFilterCriteria {
    Header(HeaderCriteria),
    Query(QueryCriteria),
    Path(Vec<PartCriteriaOperator>),
    Scheme(Vec<PartCriteriaOperator>),
    Method(Vec<PartCriteriaOperator>),
}

#[derive(Debug, Clone)]
pub enum FilterCriteria {
    Logical(LogicalFilterCriteria),
    PartFilterCriteria(PartFilterCriteria),
}

#[derive(Debug, Clone)]
pub enum LogicalFilterCriteria {
    And(Vec<PartFilterCriteria>),
    Or(Vec<PartFilterCriteria>),
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub criteria_list: Vec<FilterCriteria>,
}

impl TryFrom<&Query> for Filter {
    type Error = Box<Error>;

    fn try_from(value: &Query) -> Result<Self, Self::Error> {
        query2filter(value)
    }
}
