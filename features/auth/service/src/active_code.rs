use uuid::Uuid;

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};
use shared_shared_data_error::app::AppError;

use features_auth_entities::active_code::ActiveCodeForUpdateDto;
use features_auth_repo::active_code::{mutation::ActiveCodeMutation, query::ActiveCodeQuery};

pub struct ActiveCodeService;

impl ActiveCodeService {
    pub async fn mark_as_sent(user_id: Uuid, code: String) -> Result<bool> {
        let filters = vec![
            FilterEnum::Uuid(FilterParam {
                name: "user_id".to_string(),
                operator: FilterOperator::Equal,
                value: Some(user_id),
                raw_value: user_id.to_string(),
            }),
            FilterEnum::String(FilterParam {
                name: "code".to_string(),
                operator: FilterOperator::Equal,
                value: Some(code.clone()),
                raw_value: code,
            }),
            FilterEnum::Bool(FilterParam {
                name: "is_sent".to_string(),
                operator: FilterOperator::Equal,
                value: Some(false),
                raw_value: "false".to_string(),
            }),
        ];

        let result = ActiveCodeQuery::search(
            &Pagination::new(1, 1),
            &Order::default(),
            &FilterCondition::from(filters),
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        if result.result.is_empty() {
            return Ok(false);
        }

        let active_code = &result.result[0];
        let id = active_code.id.unwrap();

        ActiveCodeMutation::update(
            id,
            ActiveCodeForUpdateDto {
                is_used: None,
                is_sent: Some(true),
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        Ok(true)
    }
}
