use rocket::serde::json::Json;
use rocket::State;

use crate::case_details::case_info::CaseInfo;
use crate::create_case::create_case_req::CreateCaseRequest;
use crate::create_case::service;
use crate::error::Error;
use crate::user_details::User;
use crate::{AppState, DB};

#[tracing::instrument(skip(db, state, create_case_req, user))]
#[post("/", data = "<create_case_req>")]
pub async fn create_case(
    db: &DB,
    state: &State<AppState>,
    user: User,
    create_case_req: Json<CreateCaseRequest>,
) -> Result<Json<CaseInfo>, Error> {
    Ok(Json(
        service::create_case(db, state, user, create_case_req.into_inner()).await?,
    ))
}
