use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[derive(Clone)]
pub struct MailClient {
    pub client: AsyncSmtpTransport<Tokio1Executor>,
}

impl MailClient {
    pub fn new(username: String, password: String) -> Self {
        let client = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(Credentials::new(username, password))
            .build();

        Self { client }
    }
}
