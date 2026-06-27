use chrono::Utc;
use tracing::debug;
use uuid::Uuid;

use features_auth_model::login::{LoginCodeResponse, LoginData, LoginRequest};

use shared_shared_data_app::result::Result;
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};
use shared_shared_data_error::app::AppError;

use features_auth_entities::active_code::ActiveCodeForUpdateDto;
use features_auth_repo::{
    active_code::{mutation::ActiveCodeMutation, query::ActiveCodeQuery},
    auth_code::AuthCodeQuery,
    user::UserQuery,
};

pub struct LoginService {}

impl LoginService {
    pub async fn login<'a>(request: LoginRequest) -> Result<LoginData> {
        let user_data =
            UserQuery::get_user_by_email_and_password(request.email, request.password).await?;

        debug!("User data {:?}", user_data);

        let result = LoginData {
            code: "deprecated".to_string(),
        };
        Ok(result)
    }

    pub async fn verify_login_code(user_id: Uuid, login_code: String) -> Result<LoginCodeResponse> {
        // Find the login code
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
                value: Some(login_code.clone()),
                raw_value: login_code,
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
                entity: "login_code".to_string(),
            });
        }

        let active_code = &result.result[0];
        let id = active_code.id.unwrap();
        let expiration_time = active_code.expiration_time.unwrap();

        if Utc::now().naive_utc() > expiration_time {
            return Err(AppError::Internal("login_code_expired".to_string()));
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

        Ok(LoginCodeResponse {
            auth_code: auth_code_data.code.clone().unwrap_or_default(),
            redirect_uri: auth_code_data.redirect_uri.clone().unwrap_or_default(),
        })
    }
}
