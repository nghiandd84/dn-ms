use chrono::Utc;
use tracing::debug;
use uuid::Uuid;

use shared_shared_app::event_task::producer::{Producer, ProducerMessage};
use shared_shared_data_app::{
    password::hash,
    result::Result,
};
use shared_shared_data_core::{
    filter::{FilterCondition, FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::Pagination,
};
use shared_shared_data_error::{app::AppError, auth::AuthError};

use features_auth_entities::{
    active_code::ActiveCodeForCreateDto, active_code::ActiveCodeForUpdateDto,
    user::UserForPasswordChangeDto,
};
use features_auth_model::password::PasswordResponse;
use features_auth_repo::{
    active_code::{mutation::ActiveCodeMutation, query::ActiveCodeQuery},
    user::{UserMutation, UserQuery},
};
use features_auth_stream::{password::PasswordMessage, AuthMessage};

use rand::{thread_rng, Rng};

pub struct PasswordService;

impl PasswordService {
    /// Request a change password code for an authenticated user.
    /// Generates a code and sends it via Kafka (email notification).
    pub async fn request_change_password(
        producer: &Producer,
        user_id: Uuid,
    ) -> Result<PasswordResponse> {
        // Get user data for email and language
        let user = UserQuery::get_user_by_id_raw(user_id).await?;
        let email = user.email.clone().unwrap_or_default();
        let language = user.language.clone().unwrap_or_else(|| "en-US".to_string());

        // Generate change code (6 digits)
        let change_code: String = thread_rng()
            .sample_iter(&rand::distributions::Uniform::from(0..10))
            .take(6)
            .map(|n| n.to_string())
            .collect();

        // Store change code using active_code mechanism
        let active_code_dto = ActiveCodeForCreateDto {
            user_id,
            code: change_code.clone(),
        };
        ActiveCodeMutation::create(active_code_dto)
            .await
            .map_err(|_| AppError::Unknown)?;

        // Send Kafka event with change code
        let auth_message = AuthMessage::Password {
            message: PasswordMessage::ChangeRequest {
                user_id: user_id.to_string(),
                email,
                change_code,
                language_code: language,
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending change password request message to Kafka: {:?}", e.reason);
        }

        Ok(PasswordResponse {
            ok: true,
            message: "change_code_sent".to_string(),
        })
    }

    /// Change password using a change code (authenticated user).
    pub async fn change_password(
        producer: &Producer,
        user_id: Uuid,
        change_code: String,
        new_password: String,
    ) -> Result<PasswordResponse> {
        // Find the active code for this user
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
                value: Some(change_code.clone()),
                raw_value: change_code,
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
            return Err(AppError::Auth(AuthError::PasswordResetCodeNotFound));
        }

        let active_code = &result.result[0];
        let code_id = active_code.id.unwrap();
        let expiration_time = active_code.expiration_time.unwrap();

        // Check expiration
        if Utc::now().naive_utc() > expiration_time {
            return Err(AppError::Auth(AuthError::PasswordResetCodeExpired));
        }

        // Mark code as used
        ActiveCodeMutation::update(
            code_id,
            ActiveCodeForUpdateDto {
                is_used: Some(true),
                is_sent: None,
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Hash new password and update user
        let hashed_password = hash(&new_password).map_err(|_| AppError::Unknown)?;
        UserMutation::update_password(
            user_id,
            UserForPasswordChangeDto {
                password: hashed_password,
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Get user email for notification
        let user = UserQuery::get_user_by_id_raw(user_id).await?;
        let email = user.email.unwrap_or_default();

        // Send Kafka notification
        let auth_message = AuthMessage::Password {
            message: PasswordMessage::Changed {
                user_id: user_id.to_string(),
                email,
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending password changed message to Kafka: {:?}", e.reason);
        }

        Ok(PasswordResponse {
            ok: true,
            message: "password_changed".to_string(),
        })
    }

    /// Request a password reset (public). Generates a reset code and sends it via Kafka.
    pub async fn request_reset(producer: &Producer, email: String) -> Result<PasswordResponse> {
        // Find user by email
        let user = UserQuery::get_user_by_email(email.clone()).await?;
        let user_id = user.id.unwrap();
        let language = user.language.clone().unwrap_or_else(|| "en-US".to_string());

        // Generate reset code (6 digits)
        let reset_code: String = thread_rng()
            .sample_iter(&rand::distributions::Uniform::from(0..10))
            .take(6)
            .map(|n| n.to_string())
            .collect();

        // Store reset code using active_code mechanism
        let active_code_dto = ActiveCodeForCreateDto {
            user_id,
            code: reset_code.clone(),
        };
        ActiveCodeMutation::create(active_code_dto)
            .await
            .map_err(|_| AppError::Unknown)?;

        // Send Kafka event with reset code
        let auth_message = AuthMessage::Password {
            message: PasswordMessage::ResetRequest {
                user_id: user_id.to_string(),
                email: email.clone(),
                reset_code,
                language_code: language,
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending password reset request message to Kafka: {:?}", e.reason);
        }

        Ok(PasswordResponse {
            ok: true,
            message: "reset_code_sent".to_string(),
        })
    }

    /// Reset password using a reset code (public).
    pub async fn reset_password(
        producer: &Producer,
        email: String,
        reset_code: String,
        new_password: String,
    ) -> Result<PasswordResponse> {
        // Find user by email
        let user = UserQuery::get_user_by_email(email.clone()).await?;
        let user_id = user.id.unwrap();

        // Find the active code (reset code) for this user
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
                value: Some(reset_code.clone()),
                raw_value: reset_code,
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
            return Err(AppError::Auth(AuthError::PasswordResetCodeNotFound));
        }

        let active_code = &result.result[0];
        let code_id = active_code.id.unwrap();
        let expiration_time = active_code.expiration_time.unwrap();

        // Check expiration
        if Utc::now().naive_utc() > expiration_time {
            return Err(AppError::Auth(AuthError::PasswordResetCodeExpired));
        }

        // Mark code as used
        ActiveCodeMutation::update(
            code_id,
            ActiveCodeForUpdateDto {
                is_used: Some(true),
                is_sent: None,
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Hash new password and update user
        let hashed_password = hash(&new_password).map_err(|_| AppError::Unknown)?;
        UserMutation::update_password(
            user_id,
            UserForPasswordChangeDto {
                password: hashed_password,
            },
        )
        .await
        .map_err(|_| AppError::Unknown)?;

        // Send Kafka notification
        let auth_message = AuthMessage::Password {
            message: PasswordMessage::Changed {
                user_id: user_id.to_string(),
                email,
            },
        };
        let message = ProducerMessage {
            payload: auth_message,
            key: None,
        };
        if let Err(e) = producer.send(&message).await {
            debug!("Error sending password changed message to Kafka: {:?}", e.reason);
        }

        Ok(PasswordResponse {
            ok: true,
            message: "password_reset".to_string(),
        })
    }
}
