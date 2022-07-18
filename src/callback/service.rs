use rocket::State;

use crate::{AppState, DB};
use crate::error::Error;
use crate::models::status::Status;

#[tracing::instrument(name = "service::get_callback_details", skip(db, state))]
pub async fn send_callback_details(db: &DB, state: &State<AppState>, case_id: i32) -> Result<Status, Error> {
    let callback_details = db.get_callback_details(case_id).await?;

    state.http_client.send_callback(callback_details).await
}
