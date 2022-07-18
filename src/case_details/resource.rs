use rocket::serde::json::Json;

use crate::case_details::case_info::CaseInfo;
use crate::case_details::service;
use crate::error::Error;
use crate::user_details::User;
use crate::DB;

#[tracing::instrument(skip(db, user))]
#[get("/<case_id>")]
pub async fn get_case_info(db: &DB, user: User, case_id: i32) -> Result<Json<CaseInfo>, Error> {
    Ok(Json(service::get_case_info(db, &user, case_id).await?))
}
