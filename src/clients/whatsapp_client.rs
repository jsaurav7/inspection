#[derive(Clone)]
pub struct WhatsappClient {
    pub client: reqwest::Client,
    pub fresh_chat_auth_token: String,
}

impl WhatsappClient {
    pub fn new(fresh_chat_auth_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            fresh_chat_auth_token,
        }
    }
}
