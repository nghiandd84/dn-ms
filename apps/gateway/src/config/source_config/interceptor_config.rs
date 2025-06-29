use std::collections::HashMap;

use serde::Deserialize;

use crate::gateway::interceptor::InterceptorType;

#[derive(Debug, Clone, Deserialize)]
pub struct InterceptorConfig {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub interceptor_type: InterceptorType,
    pub enabled: bool,
    pub filter: Option<String>,
    pub config: Option<HashMap<String, String>>,
}
