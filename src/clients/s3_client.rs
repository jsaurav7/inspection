use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

use crate::error::Error;

#[derive(Clone)]
pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-south-1");
        let aws_config = aws_config::from_env().region(region_provider).load().await;

        Self {
            client: Client::new(&aws_config),
        }
    }

    #[tracing::instrument(name = "Get File", skip(self))]
    pub async fn get_file(&self, key: String) -> Result<Vec<u8>, Error> {
        self.client
            .get_object()
            .bucket("wimwisure-production-documents")
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to get file from s3: {}", e);
                Error::S3 { source: e }
            })?
            .body
            .collect()
            .await
            .map(|b| b.into_bytes().to_vec())
            .map_err(|e| {
                error!("Failed to get file from s3: {}", e);
                Error::Unknown
            })
    }
}
