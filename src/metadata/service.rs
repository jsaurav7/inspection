use crate::error::Error;
use crate::metadata::case_metadata::CaseMetadata;
use crate::DB;

#[tracing::instrument(name = "service::get_case_metadata", skip(db))]
pub async fn get_case_metadata(db: &DB, company: &str) -> Result<CaseMetadata, Error> {
    db.get_case_metadata(company).await
}
