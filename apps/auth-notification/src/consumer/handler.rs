use std::collections::HashMap;
use tracing::{debug, error};

use features_auth_model::state::AuthAppState;
use features_auth_stream::{signup::SignUpMessage, AuthMessage};

use features_email_template_remote::EmailTemplateService;

use crate::consumer::error::ConsumerError;

pub async fn handle_consumer_message(
    message: AuthMessage,
    auth_state: AuthAppState,
    headers: Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = match message {
        AuthMessage::SignIn { message } => {
            debug!("Handling sign-in message: {:?}", message);
            Ok(())
        }
        AuthMessage::SignUp { message } => {
            debug!("Handling sign-up message");
            let _ = handle_signup_message(auth_state, message).await?;

            Ok(())
        }
    };
    result
}

async fn handle_signup_message<'a>(
    _auth_state: AuthAppState,
    message: SignUpMessage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match message {
        SignUpMessage::Success {
            user_id,
            email,
            app_key,
            active_code,
        } => {
            debug!(
                "User signed up successfully: user_id={:?}, email={}, app_key={}, active_code={}",
                user_id, email, app_key, active_code
            );
            let key = format!("{}_ACTIVE_CODE", app_key);
            let email_template = EmailTemplateService::get_email_template_by_key(key).await;
            match email_template {
                Ok(template) => {
                    debug!("Fetched email template: {:?}", template);
                    // Here you can add logic to send the email using the fetched template
                }
                Err(e) => {
                    error!("Failed to fetch email template: {}", e);
                    return Err(Box::new(ConsumerError::NotFound { message: e }));
                }
            }
        }
    }

    Ok(())
}
