use rocket::serde::json::Json;

use crate::error::Error;
use crate::metadata::case_metadata::CaseMetadata;
use crate::DB;
use crate::user_details::User;

#[tracing::instrument(skip(db, user))]
#[get("/")]
pub async fn get_case_metadata(db: &DB, user: User) -> Result<Json<CaseMetadata>, Error> {
    Ok(Json(db.get_case_metadata(&user.company_name).await?))
}
