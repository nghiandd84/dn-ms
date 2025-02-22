use crate::{gateway::interceptor::InterceptorName, qe::query::Query};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterceptorConfig {
    pub name: InterceptorName,
    pub enabled: bool,
    pub filter: Option<String>,
    pub config: Option<Query>,
    pub rewrite: Option<Query>,
    pub response: Option<Query>,
}
