use chrono::Utc;
use uuid::Uuid;

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};
use shared_shared_data_error::app::AppError;

use features_auth_entities::active_code::ActiveCodeForUpdateDto;
use features_auth_entities::user::UserForUpdateDto;
use features_auth_model::signup::SignupActiveResponse;
use features_auth_repo::active_code::{mutation::ActiveCodeMutation, query::ActiveCodeQuery};
use features_auth_repo::auth_code::AuthCodeQuery;
use features_auth_repo::user::UserMutation;

pub struct ActiveCodeService;

impl ActiveCodeService {
    pub async fn activate(user_id: Uuid, code: String) -> Result<SignupActiveResponse> {
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
                name: "is_used".to_string(),
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
            return Err(AppError::EntityNotFound {
                entity: "active_code".to_string(),
            });
        }

        let active_code = &result.result[0];
        let id = active_code.id.unwrap();
        let expiration_time = active_code.expiration_time.unwrap();

        if Utc::now().naive_utc() > expiration_time {
            return Err(AppError::Internal("active_code_expired".to_string()));
        }

        // Mark code as used
        ActiveCodeMutation::update(
            id,
            ActiveCodeForUpdateDto {
                is_used: Some(true),
                is_sent: None,
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Activate user
        UserMutation::update(
            user_id,
            UserForUpdateDto {
                email: None,
                language: None,
                password: None,
                confirmed: None,
                two_factor_enabled: None,
                is_active: Some(true),
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Find auth_code for this user
        let auth_code_filters = vec![FilterEnum::Uuid(FilterParam {
            name: "user_id".to_string(),
            operator: FilterOperator::Equal,
            value: Some(user_id),
            raw_value: user_id.to_string(),
        })];

        let auth_code_result = AuthCodeQuery::search(
            &Pagination::new(1, 1),
            &Order::default(),
            &FilterCondition::from(auth_code_filters),
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        if auth_code_result.result.is_empty() {
            return Err(AppError::EntityNotFound {
                entity: "auth_code".to_string(),
            });
        }

        let auth_code_data = &auth_code_result.result[0];

        Ok(SignupActiveResponse {
            ok: true,
            auth_code: auth_code_data.code.clone().unwrap_or_default(),
            redirect_uri: auth_code_data.redirect_uri.clone().unwrap_or_default(),
        })
    }

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
