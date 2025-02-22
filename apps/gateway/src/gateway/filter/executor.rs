use tracing::trace;

use crate::{
    error::DakiaResult,
    gateway::filter::operator::{
        FilterCriteria, LogicalCriteriaOperator, LogicalFilterCriteria, PartFilterCriteria,
        PatternOperator, RelationalOperator, SetOperator,
    },
    proxy::http::Session,
    shared::pattern_matcher::PatternMatcher,
};

use super::{
    operator::{CriteriaOperator, HeaderCriteria, PartCriteriaOperator},
    Filter,
};

fn contains_slice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn match_critera_operator(operator: &CriteriaOperator, value: Option<&[u8]>) -> DakiaResult<bool> {
    match operator {
        CriteriaOperator::Relation(relational_operator) => match relational_operator {
            RelationalOperator::Eq(qval) => match value {
                Some(value) => Ok(value == qval),
                None => Ok(false),
            },
            RelationalOperator::Ne(qval) => match value {
                Some(value) => Ok(value != qval),
                None => Ok(false),
            },
        },
        CriteriaOperator::Pattern(pattern_operator) => match pattern_operator {
            PatternOperator::Contains(qval) => match value {
                Some(value) => Ok(contains_slice(value, &qval)),
                None => Ok(false),
            },
            PatternOperator::NotContains(qval) => match value {
                Some(value) => Ok(!contains_slice(value, &qval)),
                None => Ok(false),
            },
            PatternOperator::StartsWith(qval) => match value {
                Some(value) => Ok(value.starts_with(&qval)),
                None => Ok(false),
            },
            PatternOperator::NotStartWith(qval) => match value {
                Some(value) => Ok(!value.starts_with(&qval)),
                None => Ok(false),
            },
            PatternOperator::EndsWith(qval) => match value {
                Some(value) => Ok(value.ends_with(&qval)),
                None => Ok(false),
            },
            PatternOperator::NotEndsWith(qval) => match value {
                Some(value) => Ok(!value.ends_with(&qval)),
                None => Ok(false),
            },
            PatternOperator::Matches(pcre2_pattern_matcher) => match value {
                Some(value) => {
                    // TODO: return error here if something is wrong instead of returning false
                    let result = pcre2_pattern_matcher.is_match(value).unwrap_or(false);
                    Ok(result)
                }
                None => Ok(false),
            },
        },
        CriteriaOperator::Set(set_operator) => match value {
            Some(value) => match set_operator {
                SetOperator::In(qval) => {
                    for q in qval {
                        if q == value {
                            return Ok(true);
                        }
                    }

                    Ok(false)
                }
                SetOperator::Nin(qval) => {
                    for q in qval {
                        if q == value {
                            return Ok(false);
                        }
                    }

                    Ok(true)
                }
            },
            None => Ok(false),
        },
        CriteriaOperator::Exists(exists) => {
            if *exists {
                Ok(value.is_some())
            } else {
                Ok(value.is_none())
            }
        }
    }
}

fn match_part_critera_operator(
    operator: &PartCriteriaOperator,
    value: Option<&[u8]>,
) -> DakiaResult<bool> {
    match operator {
        PartCriteriaOperator::CriteriaOperator(criteria_operator) => {
            return match_critera_operator(criteria_operator, value)
        }
        PartCriteriaOperator::LogicalCriteriaOperator(logical_criteria_operator) => {
            match logical_criteria_operator {
                LogicalCriteriaOperator::And(criteria_operators) => {
                    for criteria_operator in criteria_operators {
                        if !match_critera_operator(criteria_operator, value)? {
                            return Ok(false);
                        }
                    }
                    return Ok(true);
                }
                LogicalCriteriaOperator::Or(criteria_operators) => {
                    for criteria_operator in criteria_operators {
                        if match_critera_operator(criteria_operator, value)? {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
            }
        }
    }
}

fn match_part_critera_operators(
    operators: &Vec<PartCriteriaOperator>,
    value: Option<&[u8]>,
) -> DakiaResult<bool> {
    for part_criteria_operator in operators {
        if !match_part_critera_operator(part_criteria_operator, value)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn match_header<'a>(header_criteria: &HeaderCriteria, session: &Session<'a>) -> DakiaResult<bool> {
    let header_name_bytes = header_criteria.name.as_bytes();
    let header_name_utf8 = String::from_utf8_lossy(header_name_bytes).into_owned();
    let req_header_value = session.ds_req_header(&header_name_utf8)?;
    match_part_critera_operators(&header_criteria.operator, req_header_value)
}

fn match_query<'a>(header_criteria: &HeaderCriteria, session: &Session<'a>) -> DakiaResult<bool> {
    todo!()
}

fn match_path<'a>(
    criteria_operators: &Vec<PartCriteriaOperator>,
    session: &Session<'a>,
) -> DakiaResult<bool> {
    let req_path = session.ds_req_path();
    trace!("executing path match for {req_path}",);
    match_part_critera_operators(criteria_operators, Some(req_path.as_bytes()))
}

fn match_method<'a>(
    criteria_operators: &Vec<PartCriteriaOperator>,
    session: &Session<'a>,
) -> DakiaResult<bool> {
    let req_path = session.ds_req_method()?;
    match_part_critera_operators(criteria_operators, Some(req_path.as_bytes()))
}

fn exec_part_filter<'a>(
    part_filter_criteria: &PartFilterCriteria,
    session: &Session<'a>,
) -> DakiaResult<bool> {
    match part_filter_criteria {
        PartFilterCriteria::Header(header_criteria) => match_header(header_criteria, session),
        PartFilterCriteria::Query(query_criteria) => todo!(),
        PartFilterCriteria::Path(part_criteria_operators) => {
            match_path(part_criteria_operators, session)
        }
        PartFilterCriteria::Scheme(part_criteria_operators) => todo!(),
        PartFilterCriteria::Method(part_criteria_operators) => {
            match_method(part_criteria_operators, session)
        }
    }
}

pub fn exec_filter<'a>(filter: &Filter, session: &Session<'a>) -> DakiaResult<bool> {
    trace!("executing filter match for filter \n {:#?}", filter);

    for criteria in &filter.criteria_list {
        match criteria {
            FilterCriteria::Logical(logical_filter_criteria) => match logical_filter_criteria {
                LogicalFilterCriteria::And(part_filter_criterias) => {
                    for part_filter_criteria in part_filter_criterias {
                        let is_part_filter_matched =
                            exec_part_filter(part_filter_criteria, session)?;
                        if !is_part_filter_matched {
                            return Ok(false);
                        }
                    }
                    return Ok(true);
                }
                LogicalFilterCriteria::Or(part_filter_criterias) => {
                    for part_filter_criteria in part_filter_criterias {
                        let is_part_filter_matched =
                            exec_part_filter(part_filter_criteria, session)?;
                        if is_part_filter_matched {
                            return Ok(true);
                        }
                    }
                    return Ok(false);
                }
            },
            FilterCriteria::PartFilterCriteria(part_filter_criteria) => {
                trace!(
                    "executing part filter criteria match for \n {:#?}",
                    part_filter_criteria
                );
                return exec_part_filter(part_filter_criteria, session);
            }
        }
    }

    // return true if no criteria is present to indicate a match
    Ok(true)
}
