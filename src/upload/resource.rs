use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Json;

use crate::error::Error;
use crate::models::status::Status;
use crate::user_details::User;

#[derive(rocket::FromForm)]
pub struct Upload<'r> {
    file: &'r str,
    #[field(name = "fileZip")]
    file_zip: TempFile<'r>,
}

#[tracing::instrument(skip(user, upload))]
#[post("/", data = "<upload>")]
pub async fn upload_zip(user: User, upload: Form<Upload<'_>>) -> Result<Json<Status>, Error> {
    Ok(Json(Status::success()))
}
