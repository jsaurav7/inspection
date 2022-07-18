use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::State;

use crate::error::Error;
use crate::media::photo_details::PhotoDetails;
use crate::media::service;
use crate::{AppState, DB};
use crate::user_details::User;

#[derive(Responder)]
#[response(status = 200)]
pub struct File {
    pub content: Vec<u8>,
    pub content_disposition: Header<'static>,
    pub content_type: Header<'static>,
}

#[tracing::instrument(skip(db, state))]
#[get("/<key>?<file_name>")]
pub async fn get_report(
    db: &DB,
    state: &State<AppState>,
    key: &str,
    file_name: Option<&str>,
) -> Result<File, Error> {
    let (case_id, report) = service::get_report(db, state, key).await?;

    let file_name = match file_name {
        Some(file_name) => file_name.to_string(),
        None => format!("{case_id}.pdf"),
    };

    Ok(File {
        content: report,
        content_disposition: Header::new(
            "Content-Disposition",
            format!("attachment; filename={file_name}"),
        ),
        content_type: Header::new("Content-Type", "application/pdf"),
    })
}

#[tracing::instrument(skip(db, state))]
#[get("/<key>")]
pub async fn get_photos(db: &DB, state: &State<AppState>, key: &str) -> Result<File, Error> {
    let (case_id, photos) = service::get_photos(db, state, key).await?;

    Ok(File {
        content: photos,
        content_disposition: Header::new(
            "Content-Disposition",
            format!("attachment; filename={case_id}.zip"),
        ),
        content_type: Header::new("Content-Type", "application/zip"),
    })
}

#[tracing::instrument(skip(db, user))]
#[get("/<case_id>")]
pub async fn get_photos_details(db: &DB, user: User, case_id: i32) -> Result<Json<Vec<PhotoDetails>>, Error> {
    Ok(Json(service::get_photos_details(db, &user, case_id).await?))
}
