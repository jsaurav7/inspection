use std::env;
use crate::clients::http_client::HttpClient;

use crate::clients::mail_client::MailClient;
use crate::clients::s3_client::S3Client;
use crate::clients::sms_client::SmsClient;
use crate::clients::whatsapp_client::WhatsappClient;

pub mod http_client;
pub mod mail_client;
pub mod s3_client;
pub mod sms_client;
pub mod whatsapp_client;

#[derive(Clone)]
pub struct AppState {
    pub whatsapp_client: WhatsappClient,
    pub sms_client: SmsClient,
    pub mailer: MailClient,
    pub s3_client: S3Client,
    pub http_client: HttpClient,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            whatsapp_client: WhatsappClient::new(env::var("FRESH_CHAT_AUTH_TOKEN").unwrap()),
            mailer: MailClient::new(
                env::var("SMTP_USERNAME").unwrap(),
                env::var("SMTP_PASSWORD").unwrap(),
            ),
            sms_client: SmsClient::new(env::var("SMS_API_KEY").unwrap()),
            s3_client: S3Client::new().await,
            http_client: HttpClient::new(),
        }
    }
}
