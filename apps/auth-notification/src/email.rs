use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::Mailbox, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tracing_subscriber::field::debug;

use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, error};

#[derive(Debug)]
pub struct SendMail {
    from: String,
    to: String,
    subject: String,
    html: String,
    placeholder: Option<HashMap<String, String>>,
}

impl SendMail {
    pub fn new(
        from: String,
        to: String,
        subject: String,
        html: String,
        placeholder: Option<HashMap<String, String>>,
    ) -> Self {
        SendMail {
            from,
            to,
            subject,
            html,
            placeholder,
        }
    }
}

pub async fn send_email(send_mail: &SendMail) -> Result<(), &'static str> {
    let smtp_server = std::env::var("SMTP_SERVER_NAME").expect("SMTP_SERVER_NAME must be set");
    let smtp_port = std::env::var("SMTP_SERVER_PORT").expect("SMTP_SERVER_PORT must be set");
    let smtp_port = smtp_port.parse::<u16>().unwrap_or(587);
    let smtp_user =
        std::env::var("SMTP_SERVER_USER_NAME").expect("SMTP_SERVER_USER_NAME must be set");
    let smtp_password =
        std::env::var("SMTP_SERVER_PASSWORD").expect("SMTP_SERVER_PASSWORD must be set");
    debug!(
        "SMTP cofiguration: server={}, port={}, user={}, password={}",
        smtp_server, smtp_port, smtp_user, smtp_password
    );
    let from = Mailbox::from_str(send_mail.from.clone().as_str()).unwrap();
    let to = Mailbox::from_str(send_mail.to.clone().as_str()).unwrap();
    let subject = send_mail.subject.clone();
    let html = send_mail.html.clone();
    let placeholder = send_mail.placeholder.clone();
    let html = if let Some(placeholders) = placeholder {
        let mut rendered_html = html;
        for (key, value) in placeholders {
            let placeholder_tag = format!("{{{}}}", key).to_uppercase();
            debug!(
                "Replacing placeholder: {} with value: {}",
                placeholder_tag, value
            );
            rendered_html = rendered_html.replace(&placeholder_tag, &value);
        }
        rendered_html
    } else {
        html
    };

    debug!("Final HTML content: {}", html);
    let creds = Credentials::new(smtp_user, smtp_password);
    // let email = CreateEmailBaseOptions::new(from, to, subject).with_html(html.as_str());
    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(html)
        .expect("Failed to create email message");
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server.as_str())
            .map_err(|_| "Failed to create SMTP transport")?
            .credentials(creds)
            .port(smtp_port)
            .build();

    let send = mailer.send(email).await;
    match send {
        Ok(response) => {
            debug!("Email sent successfully: {:?}", response);
        }
        Err(err) => {
            error!("Failed to send email: {:?}", err);
            // return Err("Failed to send email");
        }
    };
    Ok(())
}
