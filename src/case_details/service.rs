use crate::case_details::case_info::CaseInfo;
use crate::error::Error;
use crate::DB;
use crate::user_details::User;

#[tracing::instrument(name = "service::get_case_info", skip(db, user))]
pub async fn get_case_info(db: &DB, user: &User, case_id: i32) -> Result<CaseInfo, Error> {
    db.get_case_info(user, case_id).await
}
