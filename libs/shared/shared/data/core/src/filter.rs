use chrono::NaiveDateTime as DateTime;
use serde::Deserialize;
use serde_json::Value as Json;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    Like,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    In,
    NotIn,
    StartWith,
}
impl<'de> Deserialize<'de> for FilterOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        debug!("Filter Oerator Deserialize {}", value);

        match value.as_str() {
            "eq" => Ok(FilterOperator::Equal),
            "ne" => Ok(FilterOperator::NotEqual),
            "li" => Ok(FilterOperator::Like),
            "lt" => Ok(FilterOperator::Less),
            "lte" => Ok(FilterOperator::LessEqual),
            "gt" => Ok(FilterOperator::Greater),
            "gte" => Ok(FilterOperator::GreaterEqual),
            "in" => Ok(FilterOperator::In),
            "nin" => Ok(FilterOperator::NotIn),
            "sw" => Ok(FilterOperator::StartWith),
            _ => Err(serde::de::Error::custom("Invalid filter operator value")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FilterEnum {
    String(FilterParam<String>),
    Bool(FilterParam<bool>),
    Json(FilterParam<Json>),
    I32(FilterParam<i32>),
    I64(FilterParam<i64>),
    I8(FilterParam<i8>),
    U32(FilterParam<u32>),
    U64(FilterParam<u64>),
    F32(FilterParam<f32>),
    F64(FilterParam<f64>),
    Uuid(FilterParam<Uuid>),
    DateTime(FilterParam<DateTime>),
    VecString(FilterParam<Vec<String>>),
    VecUuid(FilterParam<Vec<Uuid>>),
}

impl FilterEnum {
    pub fn get_name(self: &Self) -> String {
        match self {
            FilterEnum::String(param) => param.name.clone(),
            FilterEnum::Json(param) => param.name.clone(),
            FilterEnum::Bool(param) => param.name.clone(),
            FilterEnum::I8(param) => param.name.clone(),
            FilterEnum::I32(param) => param.name.clone(),
            FilterEnum::I64(param) => param.name.clone(),
            FilterEnum::U32(param) => param.name.clone(),
            FilterEnum::U64(param) => param.name.clone(),
            FilterEnum::Uuid(param) => param.name.clone(),
            FilterEnum::F32(param) => param.name.clone(),
            FilterEnum::F64(param) => param.name.clone(),
            FilterEnum::DateTime(param) => param.name.clone(),
            FilterEnum::VecString(param) => param.name.clone(),
            FilterEnum::VecUuid(param) => param.name.clone(),
        }
    }

    pub fn add_name_prefix(self: &mut Self, prefix: &str) {
        let prefix = format!("{}.{}", prefix, self.get_name());

        match self {
            FilterEnum::String(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Bool(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Json(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::I32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::I64(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::I8(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::U32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::U64(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::F32(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::F64(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::Uuid(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::DateTime(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::VecString(ref mut param) => {
                param.name = prefix.clone();
            }
            FilterEnum::VecUuid(ref mut param) => {
                param.name = prefix.clone();
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilterParam<T> {
    pub name: String,
    pub value: Option<T>,
    pub raw_value: String,
    pub operator: FilterOperator,
}

pub fn convert_filter_param_to_query_string<T>(filter: &FilterParam<T>) -> String {
    let operator_str = match filter.operator {
        FilterOperator::Equal => "eq",
        FilterOperator::NotEqual => "ne",
        FilterOperator::Like => "li",
        FilterOperator::Less => "lt",
        FilterOperator::LessEqual => "lte",
        FilterOperator::Greater => "gt",
        FilterOperator::GreaterEqual => "gte",
        FilterOperator::In => "in",
        FilterOperator::NotIn => "nin",
        FilterOperator::StartWith => "sw",
    };
    format!("{}={}|{}", filter.name, operator_str, filter.raw_value)
}

pub type VecString = Vec<String>;

#[derive(Debug, Clone)]
pub enum FilterCondition {
    And(Vec<FilterCondition>),
    Or(Vec<FilterCondition>),
    Leaf(FilterEnum),
}

impl FilterCondition {
    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        FilterCondition::And(conditions)
    }

    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        FilterCondition::Or(conditions)
    }

    pub fn leaf(filter: FilterEnum) -> Self {
        FilterCondition::Leaf(filter)
    }

    /// Collect all leaf FilterEnum values from the tree (flattened).
    pub fn collect_leaves(&self) -> Vec<FilterEnum> {
        match self {
            FilterCondition::And(conditions) | FilterCondition::Or(conditions) => {
                conditions.iter().flat_map(|c| c.collect_leaves()).collect()
            }
            FilterCondition::Leaf(filter) => vec![filter.clone()],
        }
    }

    /// Add a leaf filter to the current condition (appends to And/Or, wraps Leaf into And).
    pub fn push_leaf(&mut self, filter: FilterEnum) {
        match self {
            FilterCondition::And(conditions) | FilterCondition::Or(conditions) => {
                conditions.push(FilterCondition::Leaf(filter));
            }
            _ => {
                let prev = std::mem::replace(self, FilterCondition::And(vec![]));
                *self = FilterCondition::And(vec![prev, FilterCondition::Leaf(filter)]);
            }
        }
    }

    /// Convert to query string format for remote API calls.
    /// Returns filter params and the `_condition` value.
    /// Example: `("name=eq|admin&status=eq|active", "or")`
    pub fn to_query_params(&self) -> (Vec<String>, &str) {
        let leaves = self.collect_leaves();
        let params: Vec<String> = leaves
            .iter()
            .map(|f| match f {
                FilterEnum::String(p) => convert_filter_param_to_query_string(p),
                FilterEnum::Bool(p) => convert_filter_param_to_query_string(p),
                FilterEnum::I32(p) => convert_filter_param_to_query_string(p),
                FilterEnum::I64(p) => convert_filter_param_to_query_string(p),
                FilterEnum::I8(p) => convert_filter_param_to_query_string(p),
                FilterEnum::U32(p) => convert_filter_param_to_query_string(p),
                FilterEnum::U64(p) => convert_filter_param_to_query_string(p),
                FilterEnum::F32(p) => convert_filter_param_to_query_string(p),
                FilterEnum::F64(p) => convert_filter_param_to_query_string(p),
                FilterEnum::Uuid(p) => convert_filter_param_to_query_string(p),
                FilterEnum::DateTime(p) => convert_filter_param_to_query_string(p),
                FilterEnum::Json(p) => convert_filter_param_to_query_string(p),
                FilterEnum::VecString(p) => convert_filter_param_to_query_string(p),
                FilterEnum::VecUuid(p) => convert_filter_param_to_query_string(p),
            })
            .collect();
        let condition = match self {
            FilterCondition::Or(_) => "or",
            _ => "and",
        };
        (params, condition)
    }

    /// Convert to a full query string (e.g., `name=eq|admin&status=eq|active&_condition=or`).
    pub fn to_query_string(&self) -> String {
        let (params, condition) = self.to_query_params();
        let mut qs = params.join("&");
        if condition == "or" {
            if !qs.is_empty() {
                qs.push('&');
            }
            qs.push_str("_condition=or");
        }
        qs
    }
}

pub fn default_filter_logic() -> String {
    "and".to_string()
}

impl From<Vec<FilterEnum>> for FilterCondition {
    fn from(filters: Vec<FilterEnum>) -> Self {
        FilterCondition::And(filters.into_iter().map(FilterCondition::Leaf).collect())
    }
}

impl From<&Vec<FilterEnum>> for FilterCondition {
    fn from(filters: &Vec<FilterEnum>) -> Self {
        FilterCondition::And(filters.iter().cloned().map(FilterCondition::Leaf).collect())
    }
}
