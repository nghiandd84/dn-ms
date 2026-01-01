use std::collections::HashMap;
use tracing::{debug, error};

use features_auth_model::state::AuthAppState;
use features_auth_stream::{signup::SignUpMessage, AuthMessage};

use features_email_template_remote::{EmailTemplateService, TemplateTranslationService};

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
            language_code,
            client_email,
        } => {
            debug!(
                "User signed up successfully: user_id={:?}, email={}, app_key={}, active_code={}, language_code={}",
                user_id, email, app_key, active_code, language_code
            );
            let key = format!("{}_ACTIVE_CODE", app_key);
            let email_template = EmailTemplateService::get_email_template_by_key(key).await;
            let template = match email_template {
                Ok(template) => {
                    debug!("Fetched email template: {:?}", template);
                    template
                }
                Err(e) => {
                    error!("Failed to fetch email template: {}", e);
                    return Err(Box::new(ConsumerError::NotFound { message: e }));
                }
            };
            let template_id = template.get_id().ok_or_else(|| ConsumerError::NotFound {
                message: "Email template ID".to_string(),
            })?;
            debug!("Using email template ID: {}", template_id);
            let translation = TemplateTranslationService::get_template_translations(
                template_id,
                language_code.clone(),
            )
            .await;
            let translation = match translation {
                Ok(translation) => translation,
                Err(e) => {
                    error!("Failed to fetch template translation: {}", e);
                    return Err(Box::new(ConsumerError::NotFound { message: e }));
                }
            };
            debug!("Fetched template translation: {:?}", translation);

            // let mut placeholders = HashMap::new();
        }
    }

    Ok(())
}
