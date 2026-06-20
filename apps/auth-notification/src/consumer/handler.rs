use std::collections::HashMap;
use tracing::{debug, error};

use features_auth_model::state::AuthAppState;
use features_auth_remote::ActiveCodeRemoteService;
use features_auth_stream::{signup::SignUpMessage, AuthMessage};
use features_email_template_remote::{
    EmailTemplateService, TemplatePlaceholderService, TemplateTranslationService,
};

use crate::consumer::error::ConsumerError;
use crate::email::{send_email, SendMail};

pub async fn handle_consumer_message(
    message: AuthMessage,
    auth_state: AuthAppState,
    _headers: Option<HashMap<String, String>>,
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

// Needs to be traced separately due to async context
#[tracing::instrument(name = "handle_signup_message", skip(_auth_state, message))]
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

            // Atomically mark as sent — if returns false, another consumer already handled it
            let marked = ActiveCodeRemoteService::mark_as_sent(user_id, active_code.clone())
                .await
                .map_err(|e| {
                    error!("Failed to call mark_as_sent for user_id={}: {}", user_id, e);
                    ConsumerError::NotFound {
                        message: format!("mark_as_sent failed: {}", e),
                    }
                })?;

            if !marked {
                debug!("Active code already sent for user_id={}, skipping email", user_id);
                return Ok(());
            }

            // 1. Fetch email template
            let key = format!("{}_ACTIVE_CODE", app_key);
            let template =
                EmailTemplateService::get_email_template_by_key(key.clone())
                    .await
                    .map_err(|e| {
                        error!("Failed to fetch email template '{}': {}", key, e);
                        ConsumerError::NotFound {
                            message: format!("Email template '{}': {}", key, e),
                        }
                    })?;

            let template_id = template.get_id().ok_or_else(|| {
                error!("Email template '{}' has no ID", key);
                ConsumerError::NotFound {
                    message: format!("Email template '{}' has no ID", key),
                }
            })?;

            // 2. Fetch translation for user's language
            let translation = TemplateTranslationService::get_template_translations(
                template_id,
                language_code.clone(),
            )
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch translation for template_id={}, language={}: {}",
                    template_id, language_code, e
                );
                ConsumerError::NotFound {
                    message: format!("Translation (template={}, lang={}): {}", template_id, language_code, e),
                }
            })?;

            // 3. Fetch placeholders
            let placeholders =
                TemplatePlaceholderService::get_template_holder_by_template_id(template_id)
                    .await
                    .map_err(|e| {
                        error!(
                            "Failed to fetch placeholders for template_id={}: {}",
                            template_id, e
                        );
                        ConsumerError::NotFound {
                            message: format!("Placeholders (template={}): {}", template_id, e),
                        }
                    })?;

            // 4. Build placeholder map
            let mut placeholder_maps: HashMap<String, String> = HashMap::new();
            for placeholder in placeholders.iter() {
                placeholder_maps.insert(
                    placeholder.get_placeholder_key(),
                    placeholder.get_example_value(),
                );
            }
            placeholder_maps.insert("ACTIVE_CODE".to_string(), active_code.clone());

            // 5. Send email
            let send_mail = SendMail::new(
                client_email,
                email.clone(),
                translation.get_subject(),
                translation.get_body(),
                Some(placeholder_maps),
            );
            send_email(&send_mail).await.map_err(|e| {
                error!("Failed to send activation email to {}: {}", email, e);
                ConsumerError::SendEmailError {
                    message: format!("To {}: {}", email, e),
                }
            })?;

            debug!("Activation email sent successfully to {}", email);
        }
    }

    Ok(())
}
