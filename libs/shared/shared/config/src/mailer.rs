use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use std::env;

#[derive(Debug, Clone)]
pub struct Mailer {
    pub email: String,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new() -> Self {
        let host = env::var("EMAIL_HOST").unwrap();
        let email = env::var("EMAIL_USER").unwrap();
        let password = env::var("EMAIL_PASSWORD").unwrap();
        let port = env::var("EMAIL_PORT").unwrap().parse::<u16>().unwrap();
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
            .unwrap()
            .port(port)
            .credentials(Credentials::new(email.to_owned(), password))
            .build();

        Self { email, mailer }
    }

    pub async fn send_email(
        &self,
        to: String,
        subject: String,
        body: String,
    ) -> Result<(), String> {
        let message = Message::builder()
            .from(self.email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(body);

        if let Ok(msg) = message {
            match self.mailer.send(msg).await {
                Err(_) => Err("Error sending the email".to_string()),
                _ => Ok(()),
            }
        } else {
            Err("Invalid subject or body".to_string())
        }
    }
}
