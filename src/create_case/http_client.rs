use crate::clients::http_client::HttpClient;
use crate::error::Error;

impl HttpClient {
    #[tracing::instrument(name = "http_client::get_inspection_link", skip(self))]
    pub async fn get_inspection_link(&self, case_id: i32, username: &str) -> Result<String, Error> {
        self.client
            .get(format!(
                "https://qc.wimwisure.com/util/deep-link/{case_id}/{username}"
            ))
            .send()
            .await
            .map_err(|e| {
                error!("Error fetching inspection link: {}", e);
                Error::Reqwest { source: e }
            })?
            .text()
            .await
            .map_err(|e| {
                error!("Error fetching inspection link: {}", e);
                e.into()
            })
    }
}
