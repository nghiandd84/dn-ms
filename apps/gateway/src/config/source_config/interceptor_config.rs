use std::collections::HashMap;

use serde::Deserialize;

use crate::gateway::interceptor::InterceptorName;

#[derive(Debug, Clone, Deserialize)]
pub struct InterceptorConfig {
    pub name: InterceptorName,
    pub enabled: bool,
    pub filter: Option<String>,
    pub config: Option<HashMap<String, String>>,
}
