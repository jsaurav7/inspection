use rocket::serde::json::Json;
use rocket::State;

use crate::error::Error;
use crate::events::service;
use crate::models::status::Status;
use crate::{AppState, DB};

#[derive(serde::Deserialize)]
pub struct CaseEventRequest {
    case_id: i32,
    #[serde(rename = "et")]
    event_type: String,
}

#[tracing::instrument(skip(db, state, case_event_req))]
#[post("/", data = "<case_event_req>")]
pub async fn handle_event(
    db: &DB,
    state: &State<AppState>,
    case_event_req: Json<CaseEventRequest>,
) -> Result<Json<Status>, Error> {
    let status = service::handle_event(
        db,
        state,
        case_event_req.case_id,
        case_event_req.event_type.to_string(),
    )
    .await?;

    Ok(Json(status))
}
