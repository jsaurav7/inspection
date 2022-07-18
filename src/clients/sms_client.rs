#[derive(Clone)]
pub struct SmsClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl SmsClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }
}
