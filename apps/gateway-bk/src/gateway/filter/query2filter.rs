use std::vec;

use crate::{
    error::{DakiaError, DakiaResult},
    gateway::filter::operator::{
        Header, HeaderCriteria, LogicalCriteriaOperator, PatternOperator, QueryCriteria,
        RelationalOperator, SetOperator,
    },
    qe::query::{
        self, extract_bool_or_err, extract_string_or_err, extract_vec_bytes_or_err,
        extract_vec_or_err, Query, Value,
    },
    shared::pattern_matcher::Pcre2PatternMatcher,
};

use super::{
    operator::{
        CriteriaOperator, FilterCriteria, LogicalFilterCriteria, PartCriteriaOperator,
        PartFilterCriteria,
    },
    Filter,
};

const LOGICAL_OPERATOR: [&str; 2] = ["$and", "$or"];
const HTTP_PARTS: [&str; 5] = ["scheme", "path", "method", "header", "query"];

pub fn query2filter(query: &Query) -> DakiaResult<Filter> {
    let mut filter = Filter {
        criteria_list: vec![],
    };

    for (part, part_filter) in query {
        if is_logical_filter_criteria(&part) {
            let part_filter_criterias = build_part_filter_criteria_list(part_filter)?;
            let filter_criteria = if part.eq("$and") {
                FilterCriteria::Logical(LogicalFilterCriteria::And(part_filter_criterias))
            } else {
                FilterCriteria::Logical(LogicalFilterCriteria::Or(part_filter_criterias))
            };

            filter.criteria_list.push(filter_criteria);
            continue;
        }

        if is_part_filter_criteria(&part) {
            let part_filter_criteria = build_part_filter_criteria(part, part_filter)?;
            filter
                .criteria_list
                .push(FilterCriteria::PartFilterCriteria(part_filter_criteria));
            continue;
        }

        return Err(DakiaError::i_explain(format!(
            "Invalid filter param {part}"
        )));
    }

    Ok(filter)
}

fn build_part_filter_criteria_list(part_filter: &Value) -> DakiaResult<Vec<PartFilterCriteria>> {
    match part_filter {
        Value::Scaler(scaler) => Err(DakiaError::i_explain(format!(
            "Invalid part filter, map is expected found {:?}",
            scaler
        ))),
        Value::Composite(composite) => match composite {
            query::Composite::Map(hash_map) => {
                let mut part_filter_criteria_list: Vec<PartFilterCriteria> = vec![];

                for (part, filter) in hash_map {
                    let part_filter_criteria = build_part_filter_criteria(part, filter)?;
                    part_filter_criteria_list.push(part_filter_criteria);
                }

                Ok(part_filter_criteria_list)
            }
            query::Composite::Vector(vector) => Err(DakiaError::i_explain(format!(
                "Invalid part filter, map is expected found {:?}",
                vector
            ))),
        },
    }
}

fn build_part_filter_criteria(part: &str, part_filter: &Value) -> DakiaResult<PartFilterCriteria> {
    if is_part_nested(part, "header") {
        let nested_part_name = get_nested_part_name(part, "header");
        let header_criteria = HeaderCriteria {
            name: Header::from(nested_part_name.as_str()),
            operator: build_part_criteria_operator_list(part_filter)?,
        };

        return Ok(PartFilterCriteria::Header(header_criteria));
    }

    if is_part_nested(part, "query") {
        let nested_part_name = get_nested_part_name(part, "query");
        let query_criteria = QueryCriteria {
            name: nested_part_name.as_bytes().to_vec(),
            operator: build_part_criteria_operator_list(part_filter)?,
        };

        return Ok(PartFilterCriteria::Query(query_criteria));
    }

    let part_criteria_operator_list = build_part_criteria_operator_list(part_filter)?;
    if is_part(part, "path") {
        return Ok(PartFilterCriteria::Path(part_criteria_operator_list));
    }

    if is_part(part, "method") {
        return Ok(PartFilterCriteria::Method(part_criteria_operator_list));
    }

    if is_part(part, "scheme") {
        return Ok(PartFilterCriteria::Scheme(part_criteria_operator_list));
    }

    Err(DakiaError::i_explain(format!(
        "Invalid part filter {}",
        part
    )))
}

fn build_sacler_part_criteria_operator(scaler: &query::Scaler) -> PartCriteriaOperator {
    let value = match scaler {
        query::Scaler::String(strval) => strval.to_string(),
        query::Scaler::I64(intval) => intval.to_string(),
        query::Scaler::Bool(boolval) => boolval.to_string(),
    };

    PartCriteriaOperator::CriteriaOperator(CriteriaOperator::Relation(RelationalOperator::Eq(
        value.as_bytes().to_vec(),
    )))
}

fn build_set_values(val: &Value) -> DakiaResult<Vec<Vec<u8>>> {
    let vector = extract_vec_or_err(val)?;
    let mut bytes_vector: Vec<Vec<u8>> = vec![];
    for val in vector {
        let bytes = extract_vec_bytes_or_err(val)?;
        bytes_vector.push(bytes);
    }
    Ok(bytes_vector)
}

fn build_criteria_operator(key: &str, value: &Value) -> DakiaResult<CriteriaOperator> {
    let criteria_operator = match key.to_lowercase().as_str() {
        // relational operator
        "$eq" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Relation(RelationalOperator::Eq(bytes))
        }
        "$ne" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Relation(RelationalOperator::Ne(bytes))
        }

        // set operator
        "$in" => {
            let set = build_set_values(value)?;
            CriteriaOperator::Set(SetOperator::In(set))
        }
        "$nin" => {
            let set = build_set_values(value)?;
            CriteriaOperator::Set(SetOperator::Nin(set))
        }

        // pattern operator
        "$contains" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::Contains(bytes))
        }
        "$not_contains" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::NotContains(bytes))
        }
        "$starts_with" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::StartsWith(bytes))
        }
        "$not_starts_with" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::NotStartWith(bytes))
        }
        "$ends_with" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::EndsWith(bytes))
        }
        "$not_ends_with" => {
            let bytes = extract_vec_bytes_or_err(value)?;
            CriteriaOperator::Pattern(PatternOperator::NotEndsWith(bytes))
        }
        "$matches" => {
            let pattern = extract_string_or_err(value)?;
            let pattern_matcher = Pcre2PatternMatcher::build(&pattern)?;
            CriteriaOperator::Pattern(PatternOperator::Matches(pattern_matcher))
        }

        // existance operator
        "$exists" => {
            let exists = extract_bool_or_err(value)?;
            CriteriaOperator::Exists(exists)
        }

        _ => return Err(DakiaError::i_explain(format!("Invalid operator {key}"))),
    };

    Ok(criteria_operator)
}

fn build_criteria_operators(val: &Value) -> DakiaResult<Vec<CriteriaOperator>> {
    match val {
        Value::Scaler(scaler) => Err(DakiaError::i_explain(format!(
            "Invalid operator, expected a map and found {:?}",
            scaler
        ))),
        Value::Composite(composite) => match composite {
            query::Composite::Map(hash_map) => {
                let mut criteria_operators: Vec<CriteriaOperator> = vec![];

                for (k, v) in hash_map {
                    let criteria_operator = build_criteria_operator(k, v)?;
                    criteria_operators.push(criteria_operator);
                }

                Ok(criteria_operators)
            }
            query::Composite::Vector(vector) => Err(DakiaError::i_explain(format!(
                "Invalid operator, expected a map and found {:?}",
                vector
            ))),
        },
    }
}

fn build_part_criteria_operator_list(val: &Value) -> DakiaResult<Vec<PartCriteriaOperator>> {
    match val {
        Value::Scaler(scaler) => Ok(vec![build_sacler_part_criteria_operator(scaler)]),
        Value::Composite(composite) => match composite {
            query::Composite::Map(hash_map) => {
                let mut part_criteria_operators: Vec<PartCriteriaOperator> = vec![];
                for (key, operator) in hash_map {
                    if key == "$and" {
                        let operators = build_criteria_operators(operator)?;
                        let and_operator = PartCriteriaOperator::LogicalCriteriaOperator(
                            LogicalCriteriaOperator::And(operators),
                        );
                        part_criteria_operators.push(and_operator);
                    } else if key == "$or" {
                        let operators = build_criteria_operators(operator)?;
                        let and_operator = PartCriteriaOperator::LogicalCriteriaOperator(
                            LogicalCriteriaOperator::Or(operators),
                        );
                        part_criteria_operators.push(and_operator);
                    } else {
                        let criteria_operator = build_criteria_operator(key, operator)?;
                        let part_criteria_operator =
                            PartCriteriaOperator::CriteriaOperator(criteria_operator);
                        part_criteria_operators.push(part_criteria_operator);
                    }
                }
                return Ok(part_criteria_operators);
            }
            query::Composite::Vector(vector) => {
                return Err(DakiaError::i_explain(format!(
                    "Invalid operator {:?}",
                    vector
                )))
            }
        },
    }
}
fn is_logical_filter_criteria(key: &str) -> bool {
    LOGICAL_OPERATOR.contains(&key)
}

fn is_part_filter_criteria(key: &str) -> bool {
    key.starts_with("ds.")
        || key.starts_with("req.")
        || key.starts_with("header.")
        || HTTP_PARTS.contains(&key)
}

fn is_part_nested(part_path: &str, http_part: &str) -> bool {
    part_path.starts_with(format!("ds.req.{http_part}.").as_str())
        || part_path.starts_with(format!("req.{http_part}.").as_str())
        || part_path.starts_with(format!("{http_part}.").as_str())
}

fn get_nested_part_name(part_path: &str, http_part: &str) -> String {
    if part_path.starts_with(format!("ds.req.{http_part}.").as_str()) {
        part_path.replace(format!("ds.req.{http_part}.").as_str(), "")
    } else if part_path.starts_with(format!("req.{http_part}.").as_str()) {
        part_path.replace(format!("req.{http_part}.").as_str(), "")
    } else if part_path.starts_with(format!("{http_part}.").as_str()) {
        part_path.replace(format!("{http_part}.").as_str(), "")
    } else {
        // it will never occur
        "".to_string()
    }
}

fn is_part(part_path: &str, http_part: &str) -> bool {
    part_path.starts_with(format!("ds.req.{http_part}").as_str())
        || part_path.starts_with(format!("req.{http_part}").as_str())
        || part_path.starts_with(format!("{http_part}").as_str())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_filter() {
        let yaml = r#"
            $or:
                ds.req.method: GET
                path:
                    $or:
                        $eq: /hello
                        $matches: bolo
                        $starts_with: fuck
                    $and:
                        $ends_with: fuck
            ds.req.method: GET
            $and:
                scheme:
                    $or:
                        $ne: https
                        $in:
                            - http
                            - https
            $and:
                header.content-type:
                    $contains: application/json
            scheme:
                $matches: https
        "#;

        let query: Query = serde_yaml::from_str(yaml).unwrap();
        let filter = query2filter(&query).is_ok();
        assert!(filter);
    }
}
