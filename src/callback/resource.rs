use rocket::serde::json::Json;
use rocket::State;

use crate::callback::service;
use crate::error::Error;
use crate::models::status::Status;
use crate::{AppState, DB};

#[tracing::instrument(skip(db, state))]
#[get("/<case_id>")]
pub async fn send_callback(
    db: &DB,
    state: &State<AppState>,
    case_id: i32,
) -> Result<Json<Status>, Error> {
    Ok(Json(
        service::send_callback_details(db, state, case_id).await?,
    ))
}
