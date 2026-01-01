use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::collections::HashMap;
use tracing::{debug, error};

#[derive(Debug)]
pub struct SendMail {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    placeholder: Option<HashMap<String, String>>,
}

fn create_resend_client() -> Resend {
    let api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
    Resend::new(&api_key)
}

pub async fn send_email(send_mail: &SendMail) -> Result<(), &'static str> {
    let client = create_resend_client();
    let from = send_mail.from.clone();
    let to = send_mail.to.clone();
    let subject = send_mail.subject.clone();
    let html = send_mail.html.clone();
    let placeholder = send_mail.placeholder.clone();
    let html = if let Some(placeholders) = placeholder {
        let mut rendered_html = html;
        for (key, value) in placeholders {
            let placeholder_tag = format!("{{{}}}", key);
            rendered_html = rendered_html.replace(&placeholder_tag, &value);
        }
        rendered_html
    } else {
        html
    };

    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(html.as_str());
    let send = client.emails.send(email).await;
    match send {
        Ok(response) => {
            debug!("Email sent successfully: {:?}", response);
        }
        Err(err) => {
            error!("Failed to send email: {:?}", err);
            return Err("Failed to send email");
        }
    };
    Ok(())
}
