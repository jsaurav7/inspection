use crate::callback::callback_details::CallbackDetails;
use crate::clients::http_client::HttpClient;
use crate::error::Error;
use crate::models::status::Status;

impl HttpClient {
    #[tracing::instrument(name = "http_client::send_callback", skip(self, callback_details))]
    pub async fn send_callback(&self, callback_details: CallbackDetails) -> Result<Status, Error> {
        self.client
            .post(&callback_details.url)
            .json(&callback_details)
            .send()
            .await
            .map(|_| Status::success())
            .map_err(|e| {
                error!("Error sending callback: {}", e);
                e.into()
            })
    }
}
