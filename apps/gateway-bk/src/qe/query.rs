use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{DakiaError, DakiaResult, Error};

pub type Map = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Query = Map;

#[derive(PartialEq, Debug)]
pub enum Operator {
    And,          // logical and
    Or,           // logical or
    Eq,           // equal to
    Ne,           // not equal to
    In,           // in array
    Nin,          // not in array
    Contains,     // substring present
    NotContains,  // sub strig not present,
    StartsWith,   // text starts with
    NotStartWith, // text not starts with
    EndsWith,     // text ends with
    NotEndsWith,  // text not ends with
    Exists,       // value exists
    Matches,      // value matches specified regex
}

impl TryFrom<&str> for Operator {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "$and" => Ok(Self::And),
            "$or" => Ok(Self::Or),
            "$eq" => Ok(Self::Eq),
            "$not_eq" => Ok(Self::Ne),
            "$in" => Ok(Self::In),
            "$not_in" => Ok(Self::Nin),
            "$exists" => Ok(Self::Exists),
            "$matches" => Ok(Self::Matches),
            "$contains" => Ok(Self::Contains),
            "$not_contains" => Ok(Self::NotContains),
            "$starts_with" => Ok(Self::StartsWith),
            "$not_starts_with" => Ok(Self::NotStartWith),
            "$ends_with" => Ok(Self::EndsWith),
            "$not_ends_with" => Ok(Self::NotEndsWith),
            _ => return Err(*DakiaError::create_unknown_msg("Invalid operator!")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Scaler(Scaler),
    Composite(Composite),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Scaler {
    String(String),
    I64(i64),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Composite {
    Map(Map),
    Vector(Array),
}

// fields of enum SupplierValue should be equivalent to Scaler enum fields of Query
#[derive(Debug)]
pub enum SupplierValue<'a> {
    I32(i32),
    // TODO: change Str to byte to support non UTF-8 characters
    Str(&'a str),
    None,
}

pub fn extract_key_str_or_err<'a>(query: &'a Query, qkey: &'a str) -> DakiaResult<&'a str> {
    let mismatch_err = DakiaError::i_explain(format!("mismatched value type for key {}", qkey));

    match query.get(qkey) {
        Some(qval) => match qval {
            Value::Scaler(scaler) => match scaler {
                Scaler::String(strval) => Ok(strval),
                _ => Err(mismatch_err),
            },
            _ => Err(mismatch_err),
        },
        None => Err(DakiaError::i_explain(format!(
            "Can not extract key {}",
            qkey
        ))),
    }
}

fn get_str_from_scaler(scaler: &Scaler) -> String {
    match scaler {
        Scaler::String(strval) => strval.to_string(),
        Scaler::I64(intval) => intval.to_string(),
        Scaler::Bool(boolval) => boolval.to_string(),
    }
}

pub fn extract_vec_bytes_or_err(val: &Value) -> DakiaResult<Vec<u8>> {
    match val {
        Value::Scaler(scaler) => Ok(get_str_from_scaler(scaler).as_bytes().to_vec()),
        Value::Composite(composite) => Err(DakiaError::i_explain(format!(
            "Expected a scaler value, found {:?}",
            composite
        ))),
    }
}

pub fn extract_key_vec_bytes(query: &Query, key: &str) -> DakiaResult<Option<Vec<u8>>> {
    match query.get(key) {
        Some(val) => match val {
            Value::Scaler(scaler) => Ok(Some(get_str_from_scaler(scaler).as_bytes().to_vec())),
            Value::Composite(composite) => Err(DakiaError::i_explain(format!(
                "Expected a scaler value, found {:?}",
                composite
            ))),
        },
        None => Ok(None),
    }
}

pub fn extract_vec_or_err(val: &Value) -> DakiaResult<&Vec<Value>> {
    match val {
        Value::Scaler(scaler) => Err(DakiaError::i_explain(format!(
            "Expected a vector value, found {:?}",
            scaler
        ))),
        Value::Composite(composite) => match composite {
            Composite::Map(hash_map) => Err(DakiaError::i_explain(format!(
                "Expected a vector value, found {:?}",
                hash_map
            ))),
            Composite::Vector(values) => Ok(values),
        },
    }
}

pub fn extract_string_or_err(val: &Value) -> DakiaResult<String> {
    match val {
        Value::Scaler(scaler) => Ok(get_str_from_scaler(scaler)),
        Value::Composite(composite) => Err(DakiaError::i_explain(format!(
            "Expected a string value, found {:?}",
            composite
        ))),
    }
}

pub fn extract_bool_or_err(val: &Value) -> DakiaResult<bool> {
    match val {
        Value::Scaler(scaler) => match scaler {
            Scaler::Bool(boolval) => Ok(boolval.to_owned()),
            _ => Err(DakiaError::i_explain(format!(
                "Expected a boolean value, {:?}",
                scaler
            ))),
        },
        Value::Composite(composite) => Err(DakiaError::i_explain(format!(
            "Expected a boolean value, found {:?}",
            composite
        ))),
    }
}

pub fn extract_key_i64_or_err(query: &Query, key: &str) -> DakiaResult<i64> {
    match query.get(key) {
        Some(val) => match val {
            Value::Scaler(scaler) => match scaler {
                Scaler::String(string) => Err(DakiaError::i_explain(format!(
                    "Key '{key}' expected an integer but found {:?}",
                    string
                ))),
                Scaler::I64(i64) => Ok(*i64),
                Scaler::Bool(boolean) => Err(DakiaError::i_explain(format!(
                    "Key '{key}' expected an integer but found {:?}",
                    boolean
                ))),
            },
            Value::Composite(composite) => Err(DakiaError::i_explain(format!(
                "Key '{key}' expected an integer but found {:?}",
                composite
            ))),
        },
        None => Err(DakiaError::i_explain(format!(
            "Key '{key}' expected an integer but found nothing",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_try_from() {
        assert_eq!(Operator::try_from("$and").unwrap(), Operator::And);
        assert_eq!(Operator::try_from("$or").unwrap(), Operator::Or);
        assert_eq!(Operator::try_from("$eq").unwrap(), Operator::Eq);
        assert_eq!(Operator::try_from("$not_eq").unwrap(), Operator::Ne);
        assert_eq!(Operator::try_from("$in").unwrap(), Operator::In);
        assert_eq!(Operator::try_from("$not_in").unwrap(), Operator::Nin);
        assert_eq!(Operator::try_from("$exists").unwrap(), Operator::Exists);
        assert_eq!(Operator::try_from("$matches").unwrap(), Operator::Matches);
        assert_eq!(Operator::try_from("$contains").unwrap(), Operator::Contains);
        assert_eq!(
            Operator::try_from("$not_contains").unwrap(),
            Operator::NotContains
        );
        assert_eq!(
            Operator::try_from("$starts_with").unwrap(),
            Operator::StartsWith
        );
        assert_eq!(
            Operator::try_from("$not_starts_with").unwrap(),
            Operator::NotStartWith
        );
        assert_eq!(
            Operator::try_from("$ends_with").unwrap(),
            Operator::EndsWith
        );
        assert_eq!(
            Operator::try_from("$not_ends_with").unwrap(),
            Operator::NotEndsWith
        );

        assert!(Operator::try_from("$invalid").is_err());
    }

    #[test]
    fn test_value_serialization() {
        let string_value = Value::Scaler(Scaler::String("hello".to_string()));
        let yaml = serde_yaml::to_string(&string_value).unwrap();
        assert_eq!(yaml.trim(), "hello");
    }

    #[test]
    fn test_value_deserialization() {
        let yaml = "hello";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(value, Value::Scaler(Scaler::String("hello".to_string())));
    }

    #[test]
    fn test_composite_map() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::Scaler(Scaler::I64(42)));
        let composite = Composite::Map(map);
        let yaml = serde_yaml::to_string(&composite).unwrap();
        assert!(yaml.contains("42"));
    }

    #[test]
    fn test_composite_vector() {
        let array = Array::from([
            Value::Scaler(Scaler::Bool(true)),
            Value::Scaler(Scaler::I64(10)),
        ]);
        let composite = Composite::Vector(array);
        let yaml = serde_yaml::to_string(&composite).unwrap();
        assert!(yaml.contains("true"));
        assert!(yaml.contains("10"));
    }
}
